# Weapi 低分配实现说明

本文讨论如何把当前 `weapi` 加密流程改成“尽量少堆分配”的版本。本仓库的api参考来源是FeelUOwn，实现的初版对照了Python的实现版本。

目标不是完全零分配。对这套协议来说，完全零分配不现实，也没有必要。更合理的目标是：

- 减少中间 `String`
- 减少临时 `Vec<u8>`
- 让分配次数固定且可预测
- 把不可避免的分配收敛到最终输出阶段

## 现状里的主要分配点

当前流程大致如下：

1. `serde_json::to_vec(&data)` 生成 JSON bytes
2. 第一次 AES 后 `base64` 编成 `String`
3. 第二次 AES 再生成一个 `String`
4. RSA 前先 `hex::encode` 生成一个 `String`
5. 最终 `HashMap<String, String>` 或结构体再次持有字符串

这些分配里，最值得处理的是：

- 第一次 AES 后立刻转 `String`
- RSA 前的 `hex::encode`
- 使用 `HashMap<String, String>` 承载固定两字段输出

## 能优化到什么程度

建议目标：

- `params`：只在最终 base64 输出时分配一次
- `encSecKey`：只在最终 hex 输出时分配一次
- AES 中间态全用缓冲区复用
- 输出类型从 `HashMap<String, String>` 改成固定结构体

不建议强求的点：

- `serde_json` 序列化完全零分配
- `base64` 完全不产生输出 buffer
- `reqwest` 表单编码完全不复制

这些收益很小，但会显著恶化代码可读性。

## 推荐的低分配版本思路

## 1. 输出结构固定化

不要用：

```rust
HashMap<String, String>
```

改成：

```rust
pub struct WeapiPayload {
    pub params: String,
    pub enc_sec_key: String,
}
```

这样至少少掉：

- `HashMap` 桶分配
- 两个 key 的动态分配
- 哈希开销

## 2. AES 不要中途生成 `String`

当前逻辑通常是：

1. 明文 bytes
2. AES
3. base64 string
4. 再把 string 当 bytes 做第二次 AES

更好的策略是：

- 第一次 AES 输出到 caller 提供的缓冲区
- 再直接 base64 编码到另一个预分配缓冲区
- 第二次 AES 只消费这段 base64 bytes

也就是说，内部尽量用：

```rust
&mut [u8]
Vec<u8>
```

而不是反复：

```rust
String
format!
to_string()
```

## 3. 让调用方传入工作缓冲区

函数签名应该允许复用 buffer。

例如：

```rust
pub struct WeapiBuffers {
    pub json: Vec<u8>,
    pub aes_1: Vec<u8>,
    pub b64_1: Vec<u8>,
    pub aes_2: Vec<u8>,
    pub b64_2: String,
}
```

再提供：

```rust
pub fn encrypt_request_with_buffers<T: Serialize>(
    data: &T,
    secret_key: &[u8; 16],
    buffers: &mut WeapiBuffers,
) -> Result<WeapiPayload>
```

这样单次请求内，绝大多数临时内存都可以复用。

这在“频繁请求多个 `weapi` 接口”时比每次临时 `Vec::new()` 更合理。

然而，linux最近的新经验告诉我们，在加密通道上使用缓冲区会带来大麻烦，哈哈。（虽然我的场景应该和linux相比还是小巫见大巫了，以下是说明）。

KISS原则依旧在证明它的含金量。

### 如果要用buffer,需要考虑的问题
1. 上一次加密的数据是否残留？
    - 活用clear
    - 活用`&mut [u8]`
2. 长度和容量在实现上是否保证解耦？
3. std::mem::take，确保安全。
4. 假如是异步多线程，天塌了。
5. 生命周期？借用关系？
6. 在被ai反向vibe coding的前提下，我无法为它生成我不会的东西。


## 4. RSA 避免多余 `hex::encode`

现在常见写法是：

```rust
let hex_text = hex::encode(reversed);
let message = BigUint::from_str_radix(&hex_text, 16)?;
```

这会额外分配一个 hex 字符串。

更低分配的做法是直接：

```rust
let message = BigUint::from_bytes_be(&reversed);
```

因为 Python 原实现本质上就是把反转后的字节当作一个大整数。

这样可以少掉一次中间字符串分配。

## 5. 随机 key 生成不要走 `format!`

某些实现会：

1. 生成随机字节
2. `format!("{:02x}", b)` 拼 hex
3. 截前 16 字节作为 ASCII key

如果追求少分配，例如一些用于应对高并发或者极致性能的开源的播放器项目，都会避免反复 `format!`。

更好的做法是：

- 生成固定 8 字节随机数
- 手动把每个字节转两位 hex
- 直接写入 `[u8; 16]`

示意：

```rust
fn create_secret_key() -> [u8; 16] {
    let seed: [u8; 8] = rand::random();
    let mut out = [0u8; 16];
    const HEX: &[u8; 16] = b"0123456789abcdef";

    for (i, byte) in seed.iter().enumerate() {
        out[i * 2] = HEX[(byte >> 4) as usize];
        out[i * 2 + 1] = HEX[(byte & 0x0f) as usize];
    }

    out
}
```

这比 `format!` + `collect::<String>()` 更直接。

## 6. base64 尽量写入现有缓冲区

如果继续深挖，可以不直接让 `base64` 返回 `String`，而是先算输出长度，再写入已有 buffer。

方向类似：

```rust
let needed = base64::encoded_len(input.len(), true);
buffer.resize(needed, 0);
STANDARD.encode_slice(input, &mut buffer)?;
```

这样可以继续减少中间对象。

这个优化是有价值的，但代码复杂度会明显上升。

## 一版更合理的低分配接口

```rust
pub struct WeapiPayload {
    pub params: String,
    pub enc_sec_key: String,
}

pub struct WeapiBuffers {
    pub json: Vec<u8>,
    pub stage1: Vec<u8>,
    pub stage2: Vec<u8>,
    pub scratch: Vec<u8>,
}

pub fn encrypt_request<T: Serialize>(
    data: &T,
) -> Result<WeapiPayload>;

pub fn encrypt_request_with_buffers<T: Serialize>(
    data: &T,
    secret_key: &[u8; 16],
    buffers: &mut WeapiBuffers,
) -> Result<WeapiPayload>;
```

其中：

- 普通调用走 `encrypt_request`
- 高频路径走 `encrypt_request_with_buffers`

这样能兼顾：

- API 简单
- 优化空间明确
- 不把低层细节泄漏到业务代码

## 不值得做的优化

- 为了零拷贝改掉 `serde_json`
- 自己手写 JSON 序列化
- 自己实现 AES block mode
- 为了省一个 `String` 把整个接口写成极难读的状态机
- simd

这类优化性价比很低，也会让这坨协议代码更难维护。虽然这是学习项目，但还是不会考虑太复杂的优化。

## 值得做的优化

最值得做的只有两件事：

1. `rsa_encrypt` 改用 `BigUint::from_bytes_be`
2. 给 `encrypt_request` 增加可复用 buffer 版本

KISS原则在我这里就是先写个能跑的，然后再去做精心的优化和重构。做到这里，代码就已经比一般的“能跑版”干净很多了。

## 结论

`weapi` 这类协议不值得追求完全零分配，但值得追求：

- 固定且少量的分配
- 复用缓冲区
- 中间态不滥用 `String`

最现实的方向不是“写成最炫的底层优化”，而是把这套脏逻辑限制在一个小模块里，并把它的分配行为变得可预测。
