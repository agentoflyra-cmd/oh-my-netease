# 网易云接口清单（基于 `feeluown-netease` 源码）

本文按“实际接口”整理 `feeluown-netease` 里已经用到的网易云音乐 API，而不是按 Python 函数名整理。

每一项都包含：

- 接口路径
- 请求方式
- 主要参数
- 用途
- 源码对应函数

## 1. 基础前缀

源码里实际使用了 5 组前缀：

```text
http://music.163.com
http://music.163.com/api
http://music.163.com/weapi
http://music.163.com/weapi/v1
http://music.163.com/weapi/v3
https://music.163.com/eapi
```

对应源码常量：

```python
site_uri = 'http://music.163.com'
uri = 'http://music.163.com/api'
uri_we = 'http://music.163.com/weapi'
uri_v1 = 'http://music.163.com/weapi/v1'
uri_v3 = 'http://music.163.com/weapi/v3'
uri_e = 'https://music.163.com/eapi'
```

## 2. 登录与会话

### 2.1 用户名登录

- Method: `POST`
- URL: `http://music.163.com/api/login/`
- 参数：
  - `username`
  - `password`
  - `rememberLogin=true`

源码：

```python
action = 'http://music.163.com/api/login/'
```

对应函数：

- `API.login()`

### 2.2 手机号登录

- Method: `POST`
- URL: `http://music.163.com/api/login/cellphone/`
- 参数：
  - `phone`
  - `countrycode`
  - `password`
  - `rememberLogin=true`

源码：

```python
phone_action = 'http://music.163.com/api/login/cellphone/'
```

对应函数：

- `API.login()`

### 2.3 cookies 校验

- Method: `POST`
- URL: `http://music.163.com/api/push/init`

对应函数：

- `API.check_cookies()`
```json
{
  "account": {
    "expireTime": 1779888651166,
    "nonce": "2PTlxJIvIPbYPhtl",
    "signature": "q2m2NQMIr9z9KkTKGIr4Dg34cNk="
  },
  "code": 200
}
```

### 2.4 验证码校验

- Method: `GET`
- URL: `http://music.163.com/api/image/captcha/verify/hf`
- Query:
  - `id`
  - `captcha`

对应函数：

- `API.confirm_captcha()`

### 2.5 验证码图片

- Method: `GET`
- URL: `http://music.163.com/captcha?id={captcha_id}`

对应函数：

- `API.get_captcha_url()`

### 2.6 当前用户等级 / 反查用户

- Method: `POST`
- URL: `http://music.163.com/api/user/level`
- 参数：空对象，经 `weapi` 加密发送

对应函数：

- `API.user_level()`

```json
{
  "code": 200,
  "data": {
    "info": "60G音乐网盘免费容量$黑名单上限80$云音乐商城满100减9元优惠券$价值400云贝",
    "level": 7,
    "nextLoginCount": 100,
    "nextPlayCount": 2000,
    "nowLoginCount": 100,
    "nowPlayCount": 1248,
    "progress": 0.624,
    "userId": 1929567926
  },
  "full": false
}
```

## 3. 用户相关

### 3.1 用户资料

- Method: `POST`
- URL: `http://music.163.com/weapi/share/userprofile/info`
- 参数：
  - `userId`
- 返回值
```json
  "authStatus": 0,
  "avatarImg": "http://p4.music.126.net/u7XHdB5f4bc6RdFtfA4uMA==/109951164454241983.jpg",
  "backgroundUrl": "http://p1.music.126.net/yF5GKC__8XX0A1540jQLkA==/109951164505200956.jpg",
  "code": 200,
  "createdplCnt": 13,
  "expertTags": null,
  "nickname": "卧云的孤鸿",
  "playCount": 2248,
  "playlist": [
    {
      "adType": 0,
      "anonimous": false,
      "cloudTrackCount": 0,
      "commentThreadId": "A_PL_0_7580255366",
      "coverImgId": 109951168155996142,
      "coverImgId_str": 109951168155996142,
      "coverImgUrl": "http://p4.music.126.net/W48JqGdPOwoJLKFjewMGpQ==/109951168155996142.jpg",
      "coverStatus": 0,
      "createTime": 1660007346195,
      "creator": null,
      "description": "坚信生活的美好，保持乐观的态度，要知道革命总是无往不胜的。",
      "highQuality": false,
      "id": 7580255366,
      "name": "致最后的列宁主义者萨布林",
      "newImported": false,
      "ordered": false,
      "playCount": 128,
      "privacy": 0,
      "recommendInfo": null,
      "specialType": 0,
      "status": 0,
      "subscribed": null,
      "subscribedCount": 1,
      "subscribers": [],
      "tags": [],
      "totalDuration": 0,
      "trackCount": 11,
      "trackNumberUpdateTime": 1672300532117,
      "trackUpdateTime": 1761962276821,
      "tracks": [],
      "updateTime": 1672300532117,
      "userId": 1929567926
    },
    {
      "adType": 0,
      "anonimous": false,
      "cloudTrackCount": 0,
      "commentThreadId": "A_PL_0_8115582063",
      "coverImgId": 7740561860732856,
      "coverImgUrl": "http://p3.music.126.net/-ixbe0ZNFle0t9f21Ob7bQ==/7740561860732856.jpg",
      "coverStatus": 0,
      "createTime": 1675134059211,
      "creator": null,
      "description": null,
      "highQuality": false,
      "id": 8115582063,
      "name": "你的成分",
      "newImported": false,
      "ordered": false,
      "playCount": 127,
      "privacy": 0,
      "recommendInfo": null,
      "specialType": 0,
      "status": 0,
      "subscribed": null,
      "subscribedCount": 0,
      "subscribers": [],
      "tags": [
        "世界音乐",
        "游戏",
        "经典"
      ],
      "totalDuration": 0,
      "trackCount": 56,
      "trackNumberUpdateTime": 1761297239041,
      "trackUpdateTime": 1779101920409,
      "tracks": [],
      "updateTime": 1761297239041,
      "userId": 1929567926
    },
    {
      "adType": 0,
      "anonimous": false,
      "cloudTrackCount": 0,
      "commentThreadId": "A_PL_0_7476606487",
      "coverImgId": 2893914605754636,
      "coverImgUrl": "http://p4.music.126.net/cGtyjLew7JwUq__YCRd0TA==/2893914605754636.jpg",
      "coverStatus": 0,
      "createTime": 1654596812736,
      "creator": null,
      "description": null,
      "highQuality": false,
      "id": 7476606487,
      "name": "The Witcher3",
      "newImported": false,
      "ordered": false,
      "playCount": 40,
      "privacy": 0,
      "recommendInfo": null,
      "specialType": 0,
      "status": 0,
      "subscribed": null,
      "subscribedCount": 0,
      "subscribers": [],
      "tags": [],
      "totalDuration": 0,
      "trackCount": 14,
      "trackNumberUpdateTime": 1671201930647,
      "trackUpdateTime": 1758721806559,
      "tracks": [],
      "updateTime": 1671201930647,
      "userId": 1929567926
    },
    {
      "adType": 0,
      "anonimous": false,
      "cloudTrackCount": 0,
      "commentThreadId": "A_PL_0_3023600974",
      "coverImgId": 109951162861021519,
      "coverImgId_str": 109951162861021519,
      "coverImgUrl": "http://p3.music.126.net/LcE5VR3V0hAZS95xD5x3Pg==/109951162861021519.jpg",
      "coverStatus": 0,
      "createTime": 1570705776861,
      "creator": null,
      "description": null,
      "highQuality": false,
      "id": 3023600974,
      "name": "龙潭最爱的吉他指弹曲",
      "newImported": false,
      "ordered": false,
      "playCount": 35,
      "privacy": 0,
      "recommendInfo": null,
      "specialType": 0,
      "status": 0,
      "subscribed": null,
      "subscribedCount": 0,
      "subscribers": [],
      "tags": [],
      "totalDuration": 0,
      "trackCount": 18,
      "trackNumberUpdateTime": 1587741554349,
      "trackUpdateTime": 1767090075954,
      "tracks": [],
      "updateTime": 1587741554349,
      "userId": 1929567926
    },
    {
      "adType": 0,
      "anonimous": false,
      "cloudTrackCount": 0,
      "commentThreadId": "A_PL_0_8640113463",
      "coverImgId": 109951164922687657,
      "coverImgId_str": 109951164922687657,
      "coverImgUrl": "http://p4.music.126.net/26yZJ-89psLBV60BaKI5OQ==/109951164922687657.jpg",
      "coverStatus": 0,
      "createTime": 1691556868941,
      "creator": null,
      "description": null,
      "highQuality": false,
      "id": 8640113463,
      "name": "Lube",
      "newImported": false,
      "ordered": false,
      "playCount": 22,
      "privacy": 0,
      "recommendInfo": null,
      "specialType": 0,
      "status": 0,
      "subscribed": null,
      "subscribedCount": 0,
      "subscribers": [],
      "tags": [],
      "totalDuration": 0,
      "trackCount": 50,
      "trackNumberUpdateTime": 1691556869424,
      "trackUpdateTime": 1760620920770,
      "tracks": [],
      "updateTime": 1691556869424,
      "userId": 1929567926
    },
    {
      "adType": 0,
      "anonimous": false,
      "cloudTrackCount": 0,
      "commentThreadId": "A_PL_0_7686458295",
      "coverImgId": 109951165546981553,
      "coverImgId_str": 109951165546981553,
      "coverImgUrl": "http://p3.music.126.net/FO2L_vGoTYW99p08d1PIlQ==/109951165546981553.jpg",
      "coverStatus": 0,
      "createTime": 1665662473636,
      "creator": null,
      "description": null,
      "highQuality": false,
      "id": 7686458295,
      "name": "杨青",
      "newImported": false,
      "ordered": false,
      "playCount": 19,
      "privacy": 0,
      "recommendInfo": null,
      "specialType": 0,
      "status": 0,
      "subscribed": null,
      "subscribedCount": 0,
      "subscribers": [],
      "tags": [],
      "totalDuration": 0,
      "trackCount": 44,
      "trackNumberUpdateTime": 1665662474108,
      "trackUpdateTime": 1768992723657,
      "tracks": [],
      "updateTime": 1666865028238,
      "userId": 1929567926
    },
    {
      "adType": 0,
      "anonimous": false,
      "cloudTrackCount": 0,
      "commentThreadId": "A_PL_0_7362117192",
      "coverImgId": 109951165351505570,
      "coverImgId_str": 109951165351505570,
      "coverImgUrl": "http://p3.music.126.net/a4KYp477snGHe3ZyjVpe1w==/109951165351505570.jpg",
      "coverStatus": 0,
      "createTime": 1648890896937,
      "creator": null,
      "description": null,
      "highQuality": false,
      "id": 7362117192,
      "name": "古风...等2个",
      "newImported": false,
      "ordered": false,
      "playCount": 15,
      "privacy": 0,
      "recommendInfo": null,
      "specialType": 0,
      "status": 0,
      "subscribed": null,
      "subscribedCount": 0,
      "subscribers": [],
      "tags": [],
      "totalDuration": 0,
      "trackCount": 89,
      "trackNumberUpdateTime": 1648890897153,
      "trackUpdateTime": 1778047881811,
      "tracks": [],
      "updateTime": 1650682077232,
      "userId": 1929567926
    },
    {
      "adType": 0,
      "anonimous": false,
      "cloudTrackCount": 0,
      "commentThreadId": "A_PL_0_3155326041",
      "coverImgId": 6642149743956603,
      "coverImgUrl": "http://p4.music.126.net/9Si5J0U5r0OECRHzwK6fvg==/6642149743956603.jpg",
      "coverStatus": 0,
      "createTime": 1577688810881,
      "creator": null,
      "description": null,
      "highQuality": false,
      "id": 3155326041,
      "name": "VG莱拉经纪人ID龙潭居士的2019年度歌单",
      "newImported": false,
      "ordered": false,
      "playCount": 11,
      "privacy": 0,
      "recommendInfo": null,
      "specialType": 20,
      "status": 0,
      "subscribed": null,
      "subscribedCount": 0,
      "subscribers": [],
      "tags": [],
      "totalDuration": 0,
      "trackCount": 10,
      "trackNumberUpdateTime": 1577688810907,
      "trackUpdateTime": 1761962282261,
      "tracks": [],
      "updateTime": 1577688810907,
      "userId": 1929567926
    },
    {
      "adType": 0,
      "anonimous": false,
      "cloudTrackCount": 0,
      "commentThreadId": "A_PL_0_6829257870",
      "coverImgId": 123145302324020,
      "coverImgUrl": "http://p3.music.126.net/_6nbAnsRtSMSTwNTcMP6-g==/123145302324020.jpg",
      "coverStatus": 0,
      "createTime": 1624763113179,
      "creator": null,
      "description": null,
      "highQuality": false,
      "id": 6829257870,
      "name": "背景音乐",
      "newImported": false,
      "ordered": false,
      "playCount": 2,
      "privacy": 0,
      "recommendInfo": null,
      "specialType": 0,
      "status": 0,
      "subscribed": null,
      "subscribedCount": 0,
      "subscribers": [],
      "tags": [],
      "totalDuration": 0,
      "trackCount": 8,
      "trackNumberUpdateTime": 1625711055783,
      "trackUpdateTime": 1761962280937,
      "tracks": [],
      "updateTime": 1625711055783,
      "userId": 1929567926
    },
    {
      "adType": 0,
      "anonimous": false,
      "cloudTrackCount": 0,
      "commentThreadId": "A_PL_0_5430525253",
      "coverImgId": 76965813957070,
      "coverImgUrl": "http://p4.music.126.net/NFb92_shbD9x21gJfMIJoQ==/76965813957070.jpg",
      "coverStatus": 0,
      "createTime": 1609215598056,
      "creator": null,
      "description": null,
      "highQuality": false,
      "id": 5430525253,
      "name": "VG莱拉经纪人ID龙潭居士的2020年度歌单",
      "newImported": false,
      "ordered": false,
      "playCount": 2,
      "privacy": 0,
      "recommendInfo": null,
      "specialType": 20,
      "status": 0,
      "subscribed": null,
      "subscribedCount": 0,
      "subscribers": [],
      "tags": [],
      "totalDuration": 0,
      "trackCount": 10,
      "trackNumberUpdateTime": 1609215598125,
      "trackUpdateTime": 1761962281760,
      "tracks": [],
      "updateTime": 1609215598125,
      "userId": 1929567926
    },
    {
      "adType": 0,
      "anonimous": false,
      "cloudTrackCount": 0,
      "commentThreadId": "A_PL_0_7011782352",
      "coverImgId": 109951165498307909,
      "coverImgId_str": 109951165498307909,
      "coverImgUrl": "http://p3.music.126.net/g015MDkTibsOIEI67H9AVg==/109951165498307909.jpg",
      "coverStatus": 0,
      "createTime": 1633703495171,
      "creator": null,
      "description": null,
      "highQuality": false,
      "id": 7011782352,
      "name": "古运河之恋",
      "newImported": false,
      "ordered": false,
      "playCount": 0,
      "privacy": 0,
      "recommendInfo": null,
      "specialType": 200,
      "status": 0,
      "subscribed": null,
      "subscribedCount": 0,
      "subscribers": [],
      "tags": [],
      "totalDuration": 0,
      "trackCount": 1,
      "trackNumberUpdateTime": 1633703495524,
      "trackUpdateTime": 1633703495641,
      "tracks": [],
      "updateTime": 1633703495171,
      "userId": 1929567926
    },
    {
      "adType": 0,
      "anonimous": false,
      "cloudTrackCount": 0,
      "commentThreadId": "A_PL_0_9045561270",
      "coverImgId": 109951169113512521,
      "coverImgId_str": 109951169113512521,
      "coverImgUrl": "http://p3.music.126.net/s4A7Tl3g2gCTeiHHsLL7kg==/109951169113512521.jpg",
      "coverStatus": 0,
      "createTime": 1702953414983,
      "creator": null,
      "description": null,
      "highQuality": false,
      "id": 9045561270,
      "name": "友情",
      "newImported": false,
      "ordered": false,
      "playCount": 0,
      "privacy": 0,
      "recommendInfo": null,
      "specialType": 0,
      "status": 0,
      "subscribed": null,
      "subscribedCount": 0,
      "subscribers": [],
      "tags": [],
      "totalDuration": 0,
      "trackCount": 2,
      "trackNumberUpdateTime": 1702953497363,
      "trackUpdateTime": 1762920207358,
      "tracks": [],
      "updateTime": 1702953497363,
      "userId": 1929567926
    }
  ],
  "starPlaylist": {
    "adType": 0,
    "anonimous": false,
    "cloudTrackCount": 0,
    "commentThreadId": "A_PL_0_2914930265",
    "coverImgId": 109951165445605094,
    "coverImgId_str": 109951165445605094,
    "coverImgUrl": "http://p3.music.126.net/pncmKE4AmTCDokEqvanZ4w==/109951165445605094.jpg",
    "coverStatus": 0,
    "createTime": 1564814616675,
    "creator": null,
    "description": null,
    "highQuality": false,
    "id": 2914930265,
    "name": "卧云的孤鸿喜欢的音乐",
    "newImported": false,
    "ordered": false,
    "playCount": 1149,
    "privacy": 0,
    "recommendInfo": null,
    "specialType": 5,
    "status": 0,
    "subscribed": null,
    "subscribedCount": 0,
    "subscribers": [],
    "tags": [],
    "totalDuration": 0,
    "trackCount": 128,
    "trackNumberUpdateTime": 1762335548084,
    "trackUpdateTime": 1779366750145,
    "tracks": [],
    "updateTime": 1762335548084,
    "userId": 1929567926
  },
  "userType": 0
}
```

对应函数：

- `API.user_profile()`

### 3.2 用户歌单

- Method: `GET`
- URL: `http://music.163.com/api/user/playlist/`
- Query:
  - `offset`
  - `limit`
  - `uid`

对应函数：

- `API.user_playlists()`

### 3.3 收藏专辑

- Method: `POST`
- URL: `http://music.163.com/weapi/album/sublist`
- 参数：
  - `offset`
  - `limit`
  - `csrf_token`

对应函数：

- `API.user_favorite_albums()`

### 3.4 收藏歌手

- Method: `POST`
- URL: `http://music.163.com/weapi/artist/sublist`
- 参数：
  - `offset`
  - `limit`
  - `csrf_token`

对应函数：

- `API.user_favorite_artists()`

## 4. 搜索

### 4.1 综合搜索 / 分类搜索

- Method: `POST`
- URL: `http://music.163.com/api/search/get`
- 参数：
  - `s`
  - `type`
  - `offset`
  - `total`
  - `limit`

`type` 在源码里的映射是：

- `1` 单曲
- `10` 专辑
- `100` 歌手
- `1000` 歌单
- `1002` 用户

对应函数：

- `API.search()`

## 5. 歌曲

### 5.1 单曲详情

- Method: `GET`
- URL: `http://music.163.com/api/song/detail/`
- Query:
  - `id`
  - `ids=[id]`

对应函数：

- `API.song_detail()`

### 5.2 批量歌曲详情（旧）

- Method: `GET`
- URL: `http://music.163.com/api/song/detail`
- Query:
  - `ids=[1,2,3,...]`

对应函数：

- `API.songs_detail()`

### 5.3 批量歌曲详情（v3）

- Method: `POST`
- URL: `http://music.163.com/weapi/v3/song/detail`
- 参数：
  - `c=[{"id":xxx}, ...]`
  - `ids=[...]`

对应函数：

- `API.songs_detail_v3()`

### 5.4 播放地址

- Method: `POST`
- URL: `http://music.163.com/weapi/song/enhance/player/url`
- 参数：
  - `ids`
  - `br`
  - `csrf_token`

说明：

- `br` 是目标码率
- 服务端可能回退到更低码率
- VIP / 付费歌曲可能返回 `url=null`

对应函数：

- `API.weapi_songs_url()`

### 5.5 歌词

- Method: `GET`
- URL: `http://music.163.com/api/song/lyric`
- Query:
  - `id`
  - `lv=-1`
  - `kv=1`
  - `tv=-1`

对应函数：

- `API.get_lyric_by_songid()`

### 5.6 相似歌曲

- Method: `GET`
- URL: `http://music.163.com/api/discovery/simiSong`
- Query:
  - `songid`
  - `offset`
  - `total=true`
  - `limit`

对应函数：

- `API.get_similar_song()`

### 5.7 喜欢 / 取消喜欢

- Method: `POST`
- URL: `http://music.163.com/api/song/like`
- 参数：
  - `trackId`
  - `like=true|false`
  - `time=0`

对应函数：

- `API.set_music_favorite()`

### 5.8 播放计数

- Method: `POST`
- URL: `http://music.163.com/weapi/pl/count`
- 参数：
  - `ids`
  - `br`
  - `csrf_token`

对应函数：

- `API.accumulate_pl_count()`

## 6. 评论

### 6.1 资源评论

- Method: `POST`
- URL: `http://music.163.com/weapi/v1/resource/comments/{comment_id}`
- 参数：
  - `rid`
  - `offset=0`
  - `total=true`
  - `limit=20`
  - `csrf_token`

对应函数：

- `API.get_comment()`

## 7. MV / 视频

### 7.1 MV 详情

- Method: `GET`
- URL: `http://music.163.com/api/mv/detail`
- Query:
  - `id`

对应函数：

- `API.get_mv_detail()`

## 8. 专辑

### 8.1 专辑详情

- Method: `GET`
- URL: `http://music.163.com/api/album/{album_id}`

对应函数：

- `API.album_infos()`

### 8.2 专辑描述页抓取

- Method: `GET`
- URL: `http://music.163.com/album`
- Query:
  - `id`

说明：

- 这不是标准 JSON API
- 是抓网页 HTML 再解析 `.n-albdesc`

对应函数：

- `API.album_desc()`

## 9. 歌手

### 9.1 歌手详情

- Method: `GET`
- URL: `http://music.163.com/api/artist/{artist_id}`

对应函数：

- `API.artist_infos()`

### 9.2 歌手歌曲

- Method: `POST`
- URL: `http://music.163.com/weapi/v1/artist/songs`
- 参数：
  - `id`
  - `limit`
  - `offset`
  - `order=hot`
  - `work_type=1`
  - `private_cloud=true`

对应函数：

- `API.artist_songs()`

### 9.3 歌手专辑

- Method: `GET`
- URL: `http://music.163.com/api/artist/albums/{artist_id}`
- Query:
  - `offset`
  - `limit`

对应函数：

- `API.artist_albums()`

### 9.4 歌手描述页抓取

- Method: `GET`
- URL: `http://music.163.com/artist/desc`
- Query:
  - `id`

说明：

- 这不是标准 JSON API
- 是抓网页 HTML 再解析 `.n-artdesc`

对应函数：

- `API.artist_desc()`

## 10. 歌单

### 10.1 歌单详情

- Method: `POST`
- URL: `http://music.163.com/weapi/v3/playlist/detail`
- 参数：
  - `id`
  - `limit`
  - `offset`
  - `n`

说明：

- 返回里的 `trackIds` 包含整张歌单的歌曲 id 列表

对应函数：

- `API.playlist_detail_v3()`

### 10.2 修改歌单名

- Method: `POST`
- URL: `http://music.163.com/api/playlist/update/name`
- 参数：
  - `id`
  - `name`

对应函数：

- `API.update_playlist_name()`

### 10.3 新建歌单

- Method: `POST`
- URL: `http://music.163.com/api/playlist/create`
- 参数：
  - `uid`
  - `name`

对应函数：

- `API.new_playlist()`

### 10.4 删除歌单

- Method: `POST`
- URL: `http://music.163.com/api/playlist/delete`
- 参数：
  - `id`
  - `pid`

对应函数：

- `API.delete_playlist()`

### 10.5 歌单增删歌曲

- Method: `POST`
- URL: `http://music.163.com/api/playlist/manipulate/tracks`
- 参数：
  - `tracks`
  - `pid`
  - `trackIds`
  - `op=add|del`

对应函数：

- `API.op_music_to_playlist()`

## 11. 推荐、榜单、电台

### 11.1 私人 FM / 电台歌曲

- Method: `GET`
- URL: `http://music.163.com/api/radio/get`

对应函数：

- `API.get_radio_music()`

### 11.2 每日推荐歌曲

- Method: `POST`
- URL: `http://music.163.com/weapi/v3/discovery/recommend/songs`
- 参数：空对象

对应函数：

- `API.get_recommend_songs()`

### 11.3 推荐歌单

- Method: `POST`
- URL: `http://music.163.com/api/discovery/recommend/resource`
- 参数：空对象

对应函数：

- `API.get_recommend_playlists()`

### 11.4 排行榜

- Method: `GET`
- URL: `http://music.163.com/api/toplist`

对应函数：

- `API.list_toplist()`

## 12. 云盘

### 12.1 云盘歌曲列表

- Method: `POST`
- URL: `http://music.163.com/weapi/v1/cloud/get`
- 参数：
  - `limit`
  - `offset`

对应函数：

- `API.cloud_songs()`

### 12.2 云盘歌曲详情

- Method: `POST`
- URL: `http://music.163.com/weapi/v1/cloud/get/byids`
- 参数：
  - `songIds`

对应函数：

- `API.cloud_songs_detail()`

### 12.3 云盘歌曲删除

- Method: `POST`
- URL: `http://music.163.com/weapi/cloud/del`
- 参数：
  - `songIds`

对应函数：

- `API.cloud_songs_delete()`

### 12.4 云盘歌曲匹配

- Method: `GET`
- URL: `http://music.163.com/api/cloud/user/song/match`
- Query:
  - `songId`
  - `adjustSongId`

对应函数：

- `API.cloud_song_match()`

### 12.5 云盘上传相关

`cloud_song_upload()` 不对应单一接口，它内部串了多步请求，具体实现依赖 `cloud_helpers/cloud_api.py`。

主流程：

1. 检查文件
2. 申请 token
3. 上传对象
4. 提交云盘信息
5. 发布云盘资源

如果你后面需要，我可以把“云盘上传的每一步接口”再单独拆一份。

## 13. DJ Radio

### 13.1 订阅电台列表

- Method: `POST`
- URL: `https://music.163.com/eapi/djradio/subed/v1`

对应函数：

- `API.subscribed_djradio()`

### 13.2 电台详情

- Method: `POST`
- URL: `https://music.163.com/eapi/djradio/v2/get`

对应函数：

- `API.djradio_detail()`

### 13.3 电台节目详情

- Method: `POST`
- URL: `https://music.163.com/eapi/dj/program/detail`

对应函数：

- `API.djradio_song_detail()`

### 13.4 电台节目列表

- Method: `POST`
- URL: `https://music.163.com/eapi/v1/dj/program/byradio`
- 参数：
  - `radioId`
  - `limit`
  - `offset`
  - `asc`

对应函数：

- `API.djradio_list()`

## 14. 加密层

## 14.1 weapi 加密

以下接口依赖 `encrypt_request()` 生成：

- `/weapi/share/userprofile/info`
- `/weapi/album/sublist`
- `/weapi/artist/sublist`
- `/weapi/song/enhance/player/url`
- `/weapi/v3/song/detail`
- `/weapi/v3/playlist/detail`
- `/weapi/v1/resource/comments/{id}`
- `/weapi/v1/cloud/get`
- `/weapi/v1/cloud/get/byids`
- `/weapi/cloud/del`

源码入口：

```python
def encrypt_request(self, data):
    text = json.dumps(data)
    first_aes_key = '0CoJUm6Qyw8W8jud'
    second_aes_key = self._create_aes_key(16)
```

## 14.2 eapi 加密

以下接口依赖 `eapi_encrypt()`：

- `/eapi/djradio/subed/v1`
- `/eapi/djradio/v2/get`
- `/eapi/dj/program/detail`
- `/eapi/v1/dj/program/byradio`
- 云盘上传流程中的若干接口

源码入口：

```python
def eapi_encrypt(self, path, params):
    sign_src = b'nobody' + path + b'use' + params + b'md5forencrypt'
```

## 15. 最有价值的结论

如果你的目标是把这些接口迁移到另一个项目里，优先级应该是：

1. 基础请求层
   - cookies
   - `request()`
2. `weapi` 加密
   - `encrypt_request()`
3. `eapi` 加密
   - `eapi_encrypt()`
4. 核心业务接口
   - 搜索
   - 歌曲详情
   - 播放地址
   - 歌单详情
   - 专辑 / 歌手详情
   - 评论
5. 扩展接口
   - 推荐
   - 云盘
   - DJ Radio

如果你需要，我下一步可以继续做两件事中的任意一个：

1. 把这份文档继续补成“接口 -> 请求示例 -> 返回字段”版本
2. 直接帮你从 `feeluown-netease` 提炼出一份不带 FeelUOwn 依赖的 Rust/Python 客户端设计稿

## 16. 请求示例与关键返回字段

这一节只补“高频接口”的请求示例和关键返回字段。

约定：

- `weapi` 请求体里的 `params` 和 `encSecKey` 需要先经过加密
- `eapi` 请求体里通常只有一个 `params`
- 下面示例里的加密字段都用占位符表示

### 16.1 搜索

接口：

- `POST http://music.163.com/api/search/get`

示例：

```http
POST /api/search/get HTTP/1.1
Host: music.163.com
Content-Type: application/x-www-form-urlencoded

s=%E5%91%A8%E6%9D%B0%E4%BC%A6&type=1&offset=0&total=true&limit=30
```

关键返回字段：

- `code`
- `result.songCount`
- `result.songs[]`
- `result.songs[].id`
- `result.songs[].name`
- `result.songs[].artists`
- `result.songs[].album`
- `result.songs[].duration`
- `result.songs[].mvid`
- `result.songs[].fee`

### 16.2 单曲详情

接口：

- `GET http://music.163.com/api/song/detail/?id={id}&ids=[{id}]`

示例：

```http
GET /api/song/detail/?id=29019227&ids=[29019227] HTTP/1.1
Host: music.163.com
```

关键返回字段：

- `code`
- `songs[]`
- `songs[0].id`
- `songs[0].name`
- `songs[0].duration`
- `songs[0].album`
- `songs[0].artists`
- `songs[0].mvid`
- `songs[0].commentThreadId`
- `songs[0].fee`

### 16.3 批量歌曲详情（v3）

接口：

- `POST http://music.163.com/weapi/v3/song/detail`

示例：

```http
POST /weapi/v3/song/detail HTTP/1.1
Host: music.163.com
Content-Type: application/x-www-form-urlencoded

params=<encrypted>&encSecKey=<encrypted>
```

加密前参数结构：

```json
{
  "c": "[{\"id\":29019227},{\"id\":3027393}]",
  "ids": "[29019227,3027393]"
}
```

关键返回字段：

- `code`
- `songs[]`
- `songs[].id`
- `songs[].name`
- `songs[].dt`
- `songs[].al`
- `songs[].ar`
- `songs[].mv`
- `songs[].fee`
- `songs[].h`
- `songs[].m`
- `songs[].l`

说明：

- `h/m/l` 是不同音质的信息来源
- 真实可播放 URL 还需要再请求一次播放地址接口

### 16.4 播放地址

接口：

- `POST http://music.163.com/weapi/song/enhance/player/url`

示例：

```http
POST /weapi/song/enhance/player/url HTTP/1.1
Host: music.163.com
Content-Type: application/x-www-form-urlencoded

params=<encrypted>&encSecKey=<encrypted>
```

加密前参数结构：

```json
{
  "ids": [29019227],
  "br": 320000,
  "csrf_token": ""
}
```

关键返回字段：

- `code`
- `data[]`
- `data[0].id`
- `data[0].url`
- `data[0].br`
- `data[0].type`
- `data[0].freeTrialInfo`

说明：

- `url` 为空通常表示当前账号无权限
- `br` 可能小于请求值，表示服务端回退了码率

### 16.5 歌词

接口：

- `GET http://music.163.com/api/song/lyric?id={id}&lv=-1&kv=1&tv=-1`

示例：

```http
GET /api/song/lyric?id=29019227&lv=-1&kv=1&tv=-1 HTTP/1.1
Host: music.163.com
```

关键返回字段：

- `code`
- `lrc.lyric`
- `tlyric.lyric`
- `sgc`
- `qfy`
- `sfy`

### 16.6 歌单详情

接口：

- `POST http://music.163.com/weapi/v3/playlist/detail`

示例：

```http
POST /weapi/v3/playlist/detail HTTP/1.1
Host: music.163.com
Content-Type: application/x-www-form-urlencoded

params=<encrypted>&encSecKey=<encrypted>
```

加密前参数结构：

```json
{
  "id": "2829883282",
  "limit": 0,
  "offset": 0,
  "n": 0
}
```

关键返回字段：

- `code`
- `playlist.id`
- `playlist.name`
- `playlist.coverImgUrl`
- `playlist.description`
- `playlist.trackIds`
- `playlist.tracks`
- `playlist.commentThreadId`
- `playlist.creator`

说明：

- 源码注释明确写了 `trackIds` 会包含整张歌单的所有歌曲 id
- 实际拉大歌单时，通常用 `trackIds` 再去批量查 `song/detail`

### 16.7 专辑详情

接口：

- `GET http://music.163.com/api/album/{album_id}`

示例：

```http
GET /api/album/2960228 HTTP/1.1
Host: music.163.com
```

关键返回字段：

- `code`
- `album.id`
- `album.name`
- `album.picUrl`
- `album.artists`
- `album.songs`
- `album.size`
- `album.publishTime`
- `album.type`

### 16.8 歌手详情

接口：

- `GET http://music.163.com/api/artist/{artist_id}`

示例：

```http
GET /api/artist/6452 HTTP/1.1
Host: music.163.com
```

关键返回字段：

- `code`
- `artist`
- `artist.id`
- `artist.name`
- `artist.picUrl`
- `hotSongs`
- `more`

### 16.9 歌手歌曲

接口：

- `POST http://music.163.com/weapi/v1/artist/songs`

示例：

```http
POST /weapi/v1/artist/songs HTTP/1.1
Host: music.163.com
Content-Type: application/x-www-form-urlencoded

params=<encrypted>&encSecKey=<encrypted>
```

加密前参数结构：

```json
{
  "id": 6452,
  "limit": 50,
  "offset": 0,
  "order": "hot",
  "work_type": 1,
  "private_cloud": "true"
}
```

关键返回字段：

- `code`
- `songs`
- `total`
- `more`

### 16.10 评论

接口：

- `POST http://music.163.com/weapi/v1/resource/comments/{comment_id}`

示例：

```http
POST /weapi/v1/resource/comments/R_SO_4_29019227 HTTP/1.1
Host: music.163.com
Content-Type: application/x-www-form-urlencoded

params=<encrypted>&encSecKey=<encrypted>
```

加密前参数结构：

```json
{
  "rid": "R_SO_4_29019227",
  "offset": "0",
  "total": "true",
  "limit": "20",
  "csrf_token": ""
}
```

关键返回字段：

- `code`
- `total`
- `hotComments`
- `comments`
- `hotComments[].commentId`
- `hotComments[].content`
- `hotComments[].likedCount`
- `hotComments[].time`
- `hotComments[].user`
- `hotComments[].beReplied`

### 16.11 MV 详情

接口：

- `GET http://music.163.com/api/mv/detail?id={mvid}`

示例：

```http
GET /api/mv/detail?id=378783 HTTP/1.1
Host: music.163.com
```

关键返回字段：

- `code`
- `data.id`
- `data.name`
- `data.cover`
- `data.brs`
- `data.artists`
- `data.duration`

说明：

- `brs` 是清晰度到播放地址的映射，例如 `240/480/720/1080`

### 16.12 每日推荐歌曲

接口：

- `POST http://music.163.com/weapi/v3/discovery/recommend/songs`

示例：

```http
POST /weapi/v3/discovery/recommend/songs HTTP/1.1
Host: music.163.com
Content-Type: application/x-www-form-urlencoded

params=<encrypted>&encSecKey=<encrypted>
```

关键返回字段：

- `code`
- `data.dailySongs`
- `data.dailySongs[].id`
- `data.dailySongs[].name`
- `data.dailySongs[].dt`
- `data.dailySongs[].al`
- `data.dailySongs[].ar`

### 16.13 云盘歌曲列表

接口：

- `POST http://music.163.com/weapi/v1/cloud/get`

示例：

```http
POST /weapi/v1/cloud/get HTTP/1.1
Host: music.163.com
Content-Type: application/x-www-form-urlencoded

params=<encrypted>&encSecKey=<encrypted>
```

加密前参数结构：

```json
{
  "limit": 30,
  "offset": 0
}
```

关键返回字段：

- `code`
- `count`
- `data`
- `data[].simpleSong`
- `data[].songId`
- `data[].fileName`
- `data[].addTime`

### 16.14 DJ Radio 节目列表

接口：

- `POST https://music.163.com/eapi/v1/dj/program/byradio`

示例：

```http
POST /eapi/v1/dj/program/byradio HTTP/1.1
Host: music.163.com
Content-Type: application/x-www-form-urlencoded

params=<encrypted>
```

加密前参数结构：

```json
{
  "radioId": 12345,
  "limit": 50,
  "offset": 0,
  "asc": false
}
```

关键返回字段：

- `code`
- `count`
- `programs`
- `programs[].id`
- `programs[].mainSong`
- `programs[].dj`

## 17. weapi / eapi 加密说明

这一节单独讲实现方式，方便你在别的项目里复刻客户端。

## 17.1 weapi 请求格式

`weapi` 最终发送的是一个表单：

```text
params=<双层AES后的密文>
encSecKey=<RSA加密后的随机AES密钥>
```

源码入口：

```python
def encrypt_request(self, data):
    text = json.dumps(data)
    first_aes_key = '0CoJUm6Qyw8W8jud'
    second_aes_key = self._create_aes_key(16)
    enc_text = self._aes_encrypt(
        self._aes_encrypt(text, first_aes_key).decode('ascii'),
        second_aes_key).decode('ascii')
    enc_aes_key = self._rsa_encrypt(second_aes_key.encode('ascii'))
    return {
        'params': enc_text,
        'encSecKey': enc_aes_key,
    }
```

实现步骤：

1. 将原始参数做 `json.dumps`
2. 使用固定 key `0CoJUm6Qyw8W8jud` 做第一层 AES-CBC
3. 生成 16 字节随机 key
4. 使用这个随机 key 对第一层结果再做一层 AES-CBC
5. 将随机 key 反转后做 RSA 加密
6. 最终发送 `params` 和 `encSecKey`

### 17.1.1 AES 细节

源码：

```python
def _aes_encrypt(self, text, key):
    pad = 16 - len(text) % 16
    text = text + pad * chr(pad)
    encryptor = AES.new(bytes(key, 'utf-8'), 2, b'0102030405060708')
    enc_text = encryptor.encrypt(bytes(text, 'utf-8'))
    return base64.b64encode(enc_text)
```

结论：

- 模式：AES-CBC
- IV：`0102030405060708`
- Padding：PKCS#7 风格
- 输出：base64

### 17.1.2 RSA 细节

源码：

```python
def _rsa_encrypt(self, text):
    e = '010001'
    n = '00e0b509f6259df8642dbc35662901477df22677ec152b5ff68ace615...'
    reverse_text = text[::-1]
    encrypted_text = pow(int(binascii.hexlify(reverse_text), 16),
                         int(e, 16), int(n, 16))
    return format(encrypted_text, "x").zfill(256)
```

结论：

- 指数：`0x10001`
- 模数：网易云固定大整数
- 明文：随机 AES key 的反转字节串
- 输出：256 位十六进制字符串

## 17.2 eapi 请求格式

`eapi` 最终通常发送：

```text
params=<十六进制大写密文>
```

源码入口：

```python
def eapi_encrypt(self, path, params):
    params = json.dumps(params, separators=(',', ':')).encode()
    sign_src = b'nobody' + path + b'use' + params + b'md5forencrypt'
    m = hashlib.md5()
    m.update(sign_src)
    sign = m.hexdigest()
    aes_src = path + b'-36cd479b6b5-' + params + b'-36cd479b6b5-' + sign.encode()
    pad = 16 - len(aes_src) % 16
    aes_src = aes_src + bytearray([pad] * pad)
    crypt = AES.new(b'e82ckenh8dichen8', AES.MODE_ECB)
    ret = crypt.encrypt(aes_src)
    return binascii.b2a_hex(ret).upper()
```

实现步骤：

1. 使用紧凑 JSON 序列化参数
2. 拼接签名源：
   - `b'nobody' + path + b'use' + params + b'md5forencrypt'`
3. 计算 MD5 签名
4. 拼接 AES 明文：
   - `path + b'-36cd479b6b5-' + params + b'-36cd479b6b5-' + sign`
5. 做 PKCS#7 风格 padding
6. 用固定 key `e82ckenh8dichen8` 做 AES-ECB
7. 输出十六进制大写字符串

结论：

- 模式：AES-ECB
- key：`e82ckenh8dichen8`
- 输出：hex uppercase

## 17.3 云盘上传流程拆解

`cloud_song_upload()` 不是单一接口，而是一串接口调用。源码分布在：

- `fuo_netease/api.py`
- `fuo_netease/cloud_helpers/cloud_api.py`

### 17.3.1 检查云盘资源

- Method: `POST`
- URL: `https://music.163.com/eapi/cloud/upload/check`
- path 参与加密时使用：`/api/cloud/upload/check`

参数：

- `songId`
- `version`
- `md5`
- `length`
- `ext`
- `bitrate`
- `checkToken`

源码：

```python
url = self.uri_e + "/cloud/upload/check"
payload = self.api.eapi_encrypt(b"/api/cloud/upload/check", data)
```

### 17.3.2 申请 NOS token

- Method: `POST`
- URL: `https://music.163.com/eapi/nos/token/alloc`
- path 参与加密时使用：`/api/nos/token/alloc`

参数：

- `type`
- `nos_product`
- `md5`
- `local`
- `filename`
- `fileSize`
- `ext`
- `bucket`
- `checkToken`

源码：

```python
url = self.uri_e + "/nos/token/alloc"
payload = self.api.eapi_encrypt(b"/api/nos/token/alloc", data)
```

### 17.3.3 上传对象内容

- Method: `POST`
- URL: `http://45.127.129.8/{bucket}/{objectKey}`

Query:

- `version=1.0`
- `offset`
- `complete=true|false`

Headers:

- `x-nos-token`
- `Content-MD5`
- `Content-Type: cloudmusic`
- `Content-Length`

源码：

```python
requests.post(
    "http://45.127.129.8/%s/" % bucket + objectKey.replace("/", "%2F"),
    data=stream,
    params={...},
    headers={...},
)
```

说明：

- 这一步不是 `music.163.com` 域名
- 是直传对象存储

### 17.3.4 提交云盘资源信息

- Method: `POST`
- URL: `https://music.163.com/eapi/upload/cloud/info/v2`
- path 参与加密时使用：`/api/upload/cloud/info/v2`

参数：

- `resourceId`
- `songid`
- `md5`
- `filename`
- `song`
- `artist`
- `album`

源码：

```python
url = self.uri_e + "/upload/cloud/info/v2"
payload = self.api.eapi_encrypt(b"/api/upload/cloud/info/v2", data)
```

### 17.3.5 发布云盘资源

- Method: `POST`
- URL: `https://music.163.com/eapi/cloud/pub/v2`
- path 参与加密时使用：`/api/cloud/pub/v2`

参数：

- `songid`
- `checkToken`

源码：

```python
url = self.uri_e + "/cloud/pub/v2"
payload = self.api.eapi_encrypt(b"/api/cloud/pub/v2", data)
```

## 18. 迁移实现建议

如果你准备在新项目里自己实现客户端，最稳妥的落地顺序是：

1. 先做一个通用请求器
   - header
   - cookies
   - GET / POST 表单
2. 实现 `weapi` 加密
3. 实现 `eapi` 加密
4. 先验证 4 个核心接口
   - 搜索
   - 歌曲详情
   - 播放地址
   - 歌单详情
5. 再补评论、专辑、歌手
6. 最后再补云盘和 DJ Radio

原因：

- 搜索 / 歌曲详情 / 播放地址 是最基础闭环
- 云盘和 DJ Radio 更依赖登录态与加密实现
