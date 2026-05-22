# Rust + iced + Wayland 架构草案

目标：基于现有 [网易云 API 整理](../netease_api.md)，实现一个最小但结构正确的桌面程序。

约束：

- 使用 Rust
- UI 使用 `iced`
- Wayland 第一公民，不考虑 X11
- 程序启动后静默驻留
- 点击状态栏图标后才显示主界面
- 第一版优先保证“可学、可扩展、可跑通”

## 技术结论

- 主程序采用 `iced::daemon`
  - 启动时不创建窗口
  - 收到托盘事件后再打开主窗口
  - 关闭窗口不退出进程
- 托盘使用 `StatusNotifierItem` 协议
  - 只做 Wayland 桌面环境下常见的 SNI / D-Bus 路线
  - 不为 X11 做兼容层
- 网易云接口分三层处理
  - 加密层：`weapi` / `eapi`
  - 传输层：HTTP、Cookie、错误处理
  - 业务层：搜索、歌单、歌曲、用户

## 目录建议

```text
src/
  main.rs
  api/
    mod.rs
    client.rs
    weapi.rs
    eapi.rs
    dto.rs
    endpoints/
      mod.rs
      auth.rs
      search.rs
      song.rs
      playlist.rs
      user.rs
  app/
    mod.rs
    state.rs
    message.rs
    commands.rs
  tray/
    mod.rs
    ksni.rs
  ui/
    mod.rs
    daemon.rs
    window.rs
    widgets/
      sidebar.rs
      search_bar.rs
      song_list.rs
```

## 分层职责

### `api`

职责：

- 处理 `weapi` / `eapi` 加密
- 管理 `reqwest::Client` 和 cookies
- 暴露稳定的 Rust 接口，而不是把原始 JSON 直接泄漏到 UI

建议：

- `dto.rs` 存放接口返回结构
- `endpoints/*` 只负责单类 API
- 统一错误类型，不要让 UI 直接感知底层加密或网络细节

### `app`

职责：

- 保存全局状态
- 定义消息流
- 把 UI 操作转成异步任务

这一层可以视为前端里的 store + actions。

### `tray`

职责：

- 注册状态栏图标
- 接收点击和菜单事件
- 将事件转换为应用内部消息

不要让 UI 层直接依赖托盘实现细节，后续替换实现时会轻松很多。

### `ui`

职责：

- 根据状态渲染界面
- 不直接发 HTTP 请求
- 不持有复杂业务逻辑

这一层可以尽量保持“声明式”。

## 核心状态

建议先把状态控制在最小集合：

```rust
pub struct AppState {
    pub window_open: bool,
    pub current_user: Option<UserSummary>,
    pub search_keyword: String,
    pub search_loading: bool,
    pub search_results: Vec<SongItem>,
    pub playlists: Vec<PlaylistSummary>,
    pub toast: Option<String>,
}
```

说明：

- `window_open`：控制主窗口生命周期
- `current_user`：登录态和用户摘要
- `search_*`：首版核心交互
- `playlists`：左侧导航和首页展示
- `toast`：统一展示错误或提示

## 消息设计

建议使用 MVU 风格：

```rust
pub enum Message {
    TrayOpenRequested,
    TrayQuitRequested,
    WindowClosed,
    RestoreSession,
    SessionRestored(Result<UserSummary, AppError>),
    SearchInputChanged(String),
    SearchSubmitted,
    SearchLoaded(Result<Vec<SongItem>, AppError>),
    PlaylistsRequested,
    PlaylistsLoaded(Result<Vec<PlaylistSummary>, AppError>),
    DismissToast,
}
```

原则：

- 用户动作和异步结果分开
- UI 不关心请求细节，只处理消息
- 所有副作用都从 `update` 发出

## 事件流

### 启动

1. 进程启动
2. 初始化托盘
3. 读取本地 cookies
4. 尝试恢复登录态
5. 不打开窗口，进入后台驻留

### 点击状态栏

1. 托盘发送 `TrayOpenRequested`
2. `app::update` 判断窗口是否已存在
3. 若不存在，创建主窗口
4. 若已存在，则聚焦或显示已有窗口

### 搜索

1. 输入框更新 `search_keyword`
2. 用户提交搜索
3. `commands` 发起 `search` 请求
4. 收到 `SearchLoaded`
5. 更新列表区域

## 首版功能范围

只做以下内容：

1. 静默启动
2. 状态栏打开主界面
3. Cookie 恢复登录
4. 搜索单曲
5. 展示用户歌单
6. 展示歌曲详情和播放 URL

暂不做：

- 音频播放
- 下载
- 云盘上传
- 评论系统
- 电台
- 桌面歌词

这样可以把第一版的重点放在架构和状态流上，而不是被功能面拖垮。

## 主界面设计稿

建议做成单窗口三栏布局：

```text
+-------------------------------------------------------------+
| 搜索框                           [用户头像] [昵称] [设置]   |
+---------------+---------------------------------------------+
| 导航          | 内容区                                      |
|               |                                             |
| 首页          | 搜索结果 / 歌单内容 / 歌曲详情              |
| 我的歌单 1    |                                             |
| 我的歌单 2    |  [歌曲名]  [歌手]  [专辑]  [时长]           |
| 我的歌单 3    |  [歌曲名]  [歌手]  [专辑]  [时长]           |
|               |  [歌曲名]  [歌手]  [专辑]  [时长]           |
|               |                                             |
+---------------+---------------------------------------------+
| 状态栏：请求结果 / 错误提示 / 当前接口状态                  |
+-------------------------------------------------------------+
```

### 布局说明

- 顶栏：
  - 搜索输入框
  - 当前用户信息
  - 轻量设置入口
- 左栏：
  - 固定导航
  - 用户歌单列表
- 主内容区：
  - 首页时显示最近结果或欢迎信息
  - 搜索后显示歌曲列表
  - 点击歌曲可展开详情
- 底栏：
  - 只放状态文本，不做播放器

## 视觉方向

建议保持克制，不要一开始就模仿完整播放器：

- 浅色底，低饱和中性色
- 单一强调色，可借网易红但不要满屏使用
- 左栏和顶栏信息密度高，内容区留白
- 列表优先可读性，不追求花哨动效

这会更适合 `iced` 首版实现，也更方便你把注意力放在组件拆分和状态更新。

## 实现顺序

1. 建立 `iced::daemon` 入口
2. 接入托盘事件
3. 建立窗口打开 / 关闭流程
4. 实现 API 基础层
5. 跑通 Cookie 恢复登录
6. 跑通搜索
7. 接入歌单列表
8. 补充错误提示和空状态

## MVP 判断标准

满足以下条件即可认为第一版完成：

- 启动后不弹窗
- 能从状态栏打开主界面
- 能恢复登录态
- 能搜索歌曲并看到结果
- 能看到用户歌单
- 关闭窗口后程序继续驻留

## 后续扩展方向

- 接入真正的音频播放层
- 增加歌词页
- 增加下载和缓存
- 增加每日推荐和 FM
- 增加云盘能力

第一阶段不建议提前设计这些模块，先把主状态流做干净。
