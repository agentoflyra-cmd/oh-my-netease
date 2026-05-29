use anyhow::Result;
use serde::Serialize;

use crate::api::{
    API_URL,
    client::{Method, NeteaseClient},
    dto::{
        SearchAlbumsResponse, SearchArtistsResponse, SearchPlaylistsResponse, SearchSongsResponse,
        SearchUsersResponse,
    },
};

#[derive(Debug, Clone, Copy)]
pub enum SearchType {
    Song = 1,
    Album = 10,
    Artist = 100,
    Playlist = 1000,
    User = 1002,
}

#[derive(Debug, Serialize)]
pub struct SearchParams {
    pub s: String,
    #[serde(rename = "type")]
    pub search_type: i64,
    pub offset: i64,
    pub total: bool,
    pub limit: i64,
}

impl SearchParams {
    pub fn new(keyword: &str, search_type: SearchType, offset: i64, total: bool, limit: i64) -> Self {
        Self {
            s: keyword.to_string(),
            search_type: search_type as i64,
            offset,
            total,
            limit,
        }
    }
}

pub async fn search_songs(
    client: &NeteaseClient,
    keyword: &str,
    offset: i64,
    limit: i64,
) -> Result<SearchSongsResponse> {
    let value = client
        .helper(
            API_URL,
            "search/get",
            SearchParams::new(keyword, SearchType::Song, offset, true, limit),
            Method::Post,
        )
        .await?;
    Ok(serde_json::from_value(value)?)
}

pub async fn search_albums(
    client: &NeteaseClient,
    keyword: &str,
    offset: i64,
    limit: i64,
) -> Result<SearchAlbumsResponse> {
    let value = client
        .helper(
            API_URL,
            "search/get",
            SearchParams::new(keyword, SearchType::Album, offset, true, limit),
            Method::Post,
        )
        .await?;
    Ok(serde_json::from_value(value)?)
}

pub async fn search_artists(
    client: &NeteaseClient,
    keyword: &str,
    offset: i64,
    limit: i64,
) -> Result<SearchArtistsResponse> {
    let value = client
        .helper(
            API_URL,
            "search/get",
            SearchParams::new(keyword, SearchType::Artist, offset, true, limit),
            Method::Post,
        )
        .await?;
    Ok(serde_json::from_value(value)?)
}

pub async fn search_playlists(
    client: &NeteaseClient,
    keyword: &str,
    offset: i64,
    limit: i64,
) -> Result<SearchPlaylistsResponse> {
    let value = client
        .helper(
            API_URL,
            "search/get",
            SearchParams::new(keyword, SearchType::Playlist, offset, true, limit),
            Method::Post,
        )
        .await?;
    Ok(serde_json::from_value(value)?)
}

pub async fn search_users(
    client: &NeteaseClient,
    keyword: &str,
    offset: i64,
    limit: i64,
) -> Result<SearchUsersResponse> {
    let value = client
        .helper(
            API_URL,
            "search/get",
            SearchParams::new(keyword, SearchType::User, offset, true, limit),
            Method::Post,
        )
        .await?;
    Ok(serde_json::from_value(value)?)
}
