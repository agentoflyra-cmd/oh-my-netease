use anyhow::{anyhow, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::{
    fs::{copy, remove_file},
    path::{Path, PathBuf},
};
use walkdir::{self, WalkDir};

fn find_firefox_cookie_dbs(target_path: impl AsRef<Path>) -> Vec<PathBuf> {
    WalkDir::new(target_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_str() == Some("cookies.sqlite"))
        .filter(|e| {
            e.path()
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .map(|name| name.ends_with(".default-release"))
                .unwrap_or(false)
        })
        .map(|e| e.into_path())
        .collect()
}

pub fn get_cookies() -> Result<Vec<String>> {
    // home dir should be exist in linux
    let Some(mut target_dir) = std::env::home_dir() else {
        return Err(anyhow!("dto: get cookies: cannot get home dir!"));
    };
    target_dir.push(".config");
    target_dir.push("mozilla");
    target_dir.push("firefox");
    let db_file_paths = find_firefox_cookie_dbs(target_dir);
    let tmp_path = PathBuf::from("/tmp/cookies_copy.sqlite");
    let _ = remove_file(&tmp_path);
    let mut results: Vec<String> = Vec::new();
    // let connection = Connection::open(path)
    for db_file_path in db_file_paths {
        let _ = copy(db_file_path, &tmp_path);
        let connect = Connection::open(&tmp_path)?;
        let mut stmt = connect
            .prepare("SELECT name, value, host FROM moz_cookies WHERE host = ?1 OR host = ?2")?;
        let rows = stmt.query_map(["music.163.com", ".music.163.com"], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        let cookies = rows.collect::<Result<Vec<_>, _>>()?;
        let result = cookies
            .into_iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect::<Vec<_>>()
            .join("; ");
        results.push(result);
        let _ = remove_file(&tmp_path);
    }
    Ok(results)
}

// only cookies can be used to login
pub struct UserConfig {
    // perhaps will not exceed 256...I guess.
    pub users: usize,
    // storage cookies
    pub cookies: Vec<String>,
}

impl UserConfig {
    pub fn new() -> Result<Self> {
        let cookies = get_cookies()?;
        Ok(Self {
            users: cookies.len(),
            cookies,
        })
    }

    pub fn get_user_cookie(&self, user_choice: usize) -> Result<&str> {
        if user_choice > self.users {
            return Err(anyhow!(
                "UserConfig: connot more than the number of cookies!"
            ));
        }
        Ok(self.cookies.get(user_choice - 1).unwrap())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "expireTime")]
    pub expire_time: i64,
    pub nonce: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLevel {
    pub code: i64,
    pub data: UserLevelData,
    pub full: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLevelData {
    pub info: String,
    pub level: i64,
    #[serde(rename = "nextLoginCount")]
    pub next_login_count: i64,
    #[serde(rename = "nextPlayCount")]
    pub next_play_count: i64,
    #[serde(rename = "nowLoginCount")]
    pub now_login_count: i64,
    #[serde(rename = "nowPlayCount")]
    pub now_play_count: i64,
    pub progress: f64,
    #[serde(rename = "userId")]
    pub user_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub code: i32,
    pub nickname: String,
    #[serde(rename = "playCount")]
    pub play_count: i64,
    pub playlist: Vec<UserPlaylistSummary>,
    #[serde(rename = "starPlaylist")]
    pub star_play_list: StarPlaylist,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StarPlaylist {
    #[serde(rename = "coverImgUrl")]
    pub cover_img_url: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "playCount")]
    pub play_count: i64,
    #[serde(rename = "subscribedCount")]
    pub subscribed_count: i64,
    #[serde(rename = "trackCount")]
    pub track_count: i64,
    #[serde(rename = "userId")]
    pub user_id: i64,
}

// #[derive(Debug, Serialize, Deserialize)]
pub type UserPlaylistSummary = PlaylistDetail;
// pub struct UserPlaylistSummary {
//     pub id: i64,
//     pub name: String,
//     pub description: Option<String>,
//     #[serde(rename = "coverImgUrl")]
//     pub cover_img_url: String,
//     #[serde(rename = "trackCount")]
//     pub track_count: i64,
//     #[serde(rename = "playCount")]
//     pub play_count: i64,
//     #[serde(rename = "subscribedCount")]
//     pub subscribed_count: i64,
//     #[serde(rename = "createTime")]
//     pub create_time: i64,
//     #[serde(rename = "updateTime")]
//     pub update_time: i64,
//     #[serde(rename = "userId")]
//     pub user_id: i64,
//     pub tags: Vec<String>,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPlaylists {
    pub code: i64,
    pub playlist: Vec<UserPlaylistSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlaylistResponse {
    pub code: i64,
    pub id: i64,
    pub playlist: UserPlaylistSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistDetailResponse {
    pub code: i64,
    pub result: PlaylistDetail,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistDetail {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "coverImgUrl")]
    pub cover_img_url: String,
    #[serde(rename = "createTime")]
    pub create_time: i64,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    #[serde(rename = "playCount")]
    pub play_count: i64,
    #[serde(rename = "trackCount")]
    pub track_count: i64,
    #[serde(rename = "subscribedCount")]
    pub subscribed_count: i64,
    #[serde(rename = "shareCount")]
    pub share_count: Option<i64>,
    #[serde(rename = "commentCount")]
    pub comment_count: Option<i64>,
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub subscribed: Option<bool>,
    pub creator: Option<SearchUserSummary>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub tracks: Option<Vec<SearchPlaylistTrackSummary>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserFavoriteAlbums {
    pub code: i64,
    pub count: i64,
    pub data: Vec<FavoriteAlbumSummary>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "paidCount")]
    pub paid_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FavoriteAlbumSummary {
    pub id: i64,
    pub name: String,
    pub alias: Vec<String>,
    #[serde(rename = "transNames")]
    pub trans_names: Vec<String>,
    pub artists: Vec<AlbumArtistSummary>,
    #[serde(rename = "picUrl")]
    pub pic_url: String,
    pub size: i64,
    #[serde(rename = "subTime")]
    pub sub_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumArtistSummary {
    pub id: i64,
    #[serde(rename = "img1v1Url")]
    pub img1v1_url: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserFavoriteArtists {
    pub code: i64,
    pub count: i64,
    pub data: Vec<FavoriteArtistSummary>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FavoriteArtistSummary {
    pub id: i64,
    pub name: String,
    pub alias: Vec<String>,
    #[serde(rename = "albumSize")]
    pub album_size: i64,
    #[serde(rename = "mvSize")]
    pub mv_size: i64,
    #[serde(rename = "img1v1Url")]
    pub img1v1_url: String,
    #[serde(rename = "picUrl")]
    pub pic_url: String,
    pub info: String,
    pub trans: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchSongsResponse {
    pub code: i64,
    pub result: SearchSongsResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchSongsResult {
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "songCount")]
    pub song_count: i64,
    pub songs: Vec<SearchSongSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchSongSummary {
    pub id: i64,
    pub name: String,
    pub alias: Vec<String>,
    pub artists: Vec<SearchArtistLite>,
    pub album: SearchAlbumLite,
    pub duration: i64,
    pub fee: i64,
    #[serde(rename = "ftype")]
    pub ftype: i64,
    pub status: i64,
    #[serde(rename = "copyrightId")]
    pub copyright_id: i64,
    pub mark: i64,
    pub mvid: i64,
    pub rtype: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchArtistsResponse {
    pub code: i64,
    pub result: SearchArtistsResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchArtistsResult {
    #[serde(rename = "artistCount")]
    pub artist_count: i64,
    pub artists: Vec<SearchArtistSummary>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "hlWords")]
    pub hl_words: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchArtistSummary {
    pub id: i64,
    pub name: String,
    pub alias: Vec<String>,
    #[serde(rename = "albumSize")]
    pub album_size: i64,
    #[serde(rename = "musicSize")]
    pub music_size: i64,
    #[serde(rename = "mvSize")]
    pub mv_size: i64,
    #[serde(rename = "img1v1Url")]
    pub img1v1_url: String,
    #[serde(rename = "picUrl")]
    pub pic_url: Option<String>,
    pub trans: Option<String>,
    pub followed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAlbumsResponse {
    pub code: i64,
    pub result: SearchAlbumsResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAlbumsResult {
    #[serde(rename = "albumCount")]
    pub album_count: i64,
    pub albums: Vec<SearchAlbumSummary>,
    #[serde(rename = "hlWords")]
    pub hl_words: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAlbumSummary {
    pub id: i64,
    pub name: String,
    pub alias: Vec<String>,
    pub artists: Vec<SearchArtistDetail>,
    pub artist: SearchArtistDetail,
    #[serde(rename = "picUrl")]
    pub pic_url: String,
    #[serde(rename = "publishTime")]
    pub publish_time: i64,
    pub size: i64,
    #[serde(rename = "type")]
    pub album_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPlaylistsResponse {
    pub code: i64,
    pub result: SearchPlaylistsResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPlaylistsResult {
    #[serde(rename = "playlistCount")]
    pub playlist_count: i64,
    pub playlists: Vec<SearchPlaylistSummary>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "hlWords")]
    pub hl_words: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPlaylistSummary {
    pub id: i64,
    pub name: String,
    pub description: String,
    #[serde(rename = "coverImgUrl")]
    pub cover_img_url: String,
    #[serde(rename = "playCount")]
    pub play_count: i64,
    #[serde(rename = "trackCount")]
    pub track_count: i64,
    #[serde(rename = "bookCount")]
    pub book_count: i64,
    pub subscribed: bool,
    pub creator: SearchPlaylistCreator,
    pub track: SearchPlaylistTrackSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPlaylistCreator {
    pub nickname: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
    #[serde(rename = "userId")]
    pub user_id: i64,
    #[serde(rename = "userType")]
    pub user_type: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchPlaylistTrackSummary {
    pub id: i64,
    pub name: String,
    pub alias: Vec<String>,
    pub artists: Vec<SearchArtistDetail>,
    pub album: SearchAlbumDetail,
    pub duration: i64,
    pub fee: i64,
    #[serde(rename = "copyrightId")]
    pub copyright_id: i64,
    pub status: i64,
    pub mvid: i64,
    pub rtype: i64,
    #[serde(rename = "ftype")]
    pub ftype: i64,
    pub mark: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchUsersResponse {
    pub code: i64,
    pub result: SearchUsersResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchUsersResult {
    #[serde(rename = "userprofileCount")]
    pub userprofile_count: i64,
    pub userprofiles: Vec<SearchUserSummary>,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    #[serde(rename = "hlWords")]
    pub hl_words: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchUserSummary {
    #[serde(rename = "userId")]
    pub user_id: i64,
    pub nickname: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
    #[serde(default)]
    pub signature: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub followed: bool,
    #[serde(default)]
    pub gender: i64,
    #[serde(rename = "userType")]
    pub user_type: i64,
    #[serde(rename = "vipType")]
    pub vip_type: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchArtistLite {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchArtistDetail {
    pub id: i64,
    pub name: String,
    pub alias: Vec<String>,
    #[serde(rename = "img1v1Url")]
    pub img1v1_url: String,
    #[serde(rename = "picUrl")]
    pub pic_url: String,
    pub trans: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAlbumLite {
    pub id: i64,
    pub name: String,
    pub size: i64,
    #[serde(rename = "publishTime")]
    pub publish_time: i64,
    #[serde(rename = "copyrightId")]
    pub copyright_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchAlbumDetail {
    pub id: i64,
    pub name: String,
    pub artists: Vec<SearchArtistDetail>,
    #[serde(rename = "picUrl")]
    pub pic_url: String,
    #[serde(rename = "publishTime")]
    pub publish_time: i64,
    pub size: i64,
    #[serde(rename = "type")]
    pub album_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistInfoResponse {
    pub code: i64,
    pub artist: ArtistInfo,
    #[serde(default)]
    pub more: bool,
    #[serde(rename = "hotSongs")]
    pub hot_songs: Vec<SearchPlaylistTrackSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistInfo {
    pub id: i64,
    pub name: String,
    pub alias: Vec<String>,
    #[serde(rename = "albumSize")]
    pub album_size: i64,
    #[serde(rename = "musicSize")]
    pub music_size: i64,
    #[serde(rename = "mvSize")]
    pub mv_size: i64,
    #[serde(rename = "img1v1Url")]
    pub img1v1_url: String,
    #[serde(rename = "picUrl")]
    pub pic_url: String,
    #[serde(rename = "briefDesc")]
    pub brief_desc: String,
    pub trans: String,
    pub followed: bool,
}

#[cfg(test)]
mod test {
    use crate::api::dto::get_cookies;
    use anyhow::{anyhow, Result};

    #[ignore]
    #[test]
    fn test_read_db() -> Result<()> {
        let results = get_cookies()?;
        dbg!(&results);
        assert!(!results.is_empty());
        Err(anyhow!("err for test"))
    }
}
