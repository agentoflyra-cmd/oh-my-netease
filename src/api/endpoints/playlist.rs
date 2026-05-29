use std::{collections::HashMap, fs::Metadata};

use crate::api::{
    API_URL, WE_API,
    client::{Method, NeteaseClient},
    dto::{
        CreatePlaylistResponse, FavoriteAlbumSummary, FavoriteArtistSummary, PlaylistDetail, PlaylistDetailResponse, Status, UserFavoriteAlbums, UserFavoriteArtists, UserPlaylistSummary, UserPlaylists
    },
};
use anyhow::{anyhow, Result};
use serde_json::json;

pub async fn user_playlists(client: &NeteaseClient, uid: i64, offset: i64, limit: i64) -> Result<Vec<UserPlaylistSummary>>{
    let mut params = HashMap::new();
    params.insert("uid", uid);
    params.insert("offset", offset);
    params.insert("limit", limit);
    let value = client.helper(API_URL, "user/playlist", params, crate::api::client::Method::Get).await?;
    let user_playlists: UserPlaylists = serde_json::from_value(value)?;
    if user_playlists.code != 200 {
        return Err(anyhow!("client: user_playlists: get user_playlists failed!"));
    }
    Ok(user_playlists.playlist)
    
}

pub async fn user_favorite_albums(client: &NeteaseClient, offset: i64, limit: i64) -> Result<Vec<FavoriteAlbumSummary>> {
    let params = json!({
        "offset": offset,
        "limit": limit,
        "total": true,
        "csrf_token": client.csrf_token().unwrap_or_default(),
    });
    let json_value = client.post_weapi(WE_API, "album/sublist", params).await?;
    let user_favorite_albums: UserFavoriteAlbums = serde_json::from_value(json_value)?;
    if user_favorite_albums.code != 200 {
        return Err(anyhow!("client: user_playlists: get user_playlists failed!"));
    }
    Ok(user_favorite_albums.data)
}

pub async fn user_favorite_artists(
    client: &NeteaseClient,
    offset: i64,
    limit: i64,
) -> Result<Vec<FavoriteArtistSummary>> {
    let params = json!({
        "offset": offset,
        "limit": limit,
        "csrf_token": client.csrf_token().unwrap_or_default(),
    });
    let json_value = client.post_weapi(WE_API, "artist/sublist", params).await?;
    let user_favorite_artists: UserFavoriteArtists = serde_json::from_value(json_value)?;
    if user_favorite_artists.code != 200 {
        return Err(anyhow!(
            "client: user_favorite_artists: get user_favorite_artists failed!"
        ));
    }
    Ok(user_favorite_artists.data)
}

pub async fn detail_v3(
    client: &NeteaseClient,
    pid: i64,
    offset: i64,
    limit: i64,
) -> Result<PlaylistDetail> {
    let mut params = HashMap::new();
    params.insert("id", pid);
    params.insert("limit", limit);
    params.insert("offset", offset);
    params.insert("n", limit);
    let value = client
        .helper(API_URL, "playlist/detail", params, Method::Post)
        .await?;
    let detail: PlaylistDetailResponse = serde_json::from_value(value)?;
    if detail.code != 200 {
        return Err(anyhow!("client: detail_v3: get playlist detail failed!"));
    }
    Ok(detail.result)
}

#[derive(serde::Serialize)]
struct UpdatePlaylistNameParams<'a> {
    id: i64,
    name: &'a str,
}
// return: code: 200
pub async fn update_playlist_name(client: &NeteaseClient, pid: i64, name: &str) -> Result<()> {
    let params= UpdatePlaylistNameParams {
        id: pid,
        name
    };
    let json_value = client.helper(API_URL, "playlist/update/name", params, Method::Post).await?;
    let status: Status = serde_json::from_value(json_value)?;
    if status.code != 200 {
        return Err(anyhow!("update playlist name failed."));
    }
    println!("just send request and not process response yet.");
    Ok(())
}

pub async fn new_playlist(client: &NeteaseClient, uid: i64, name: &str) -> Result<PlaylistDetail> {
    // it's same as update play list name, so we can reuse it.
    let params = UpdatePlaylistNameParams {
        id: uid,
        name,
    };
    let json_value = client.helper(API_URL, "playlist/create", params, Method::Post).await?;
    let play_response: CreatePlaylistResponse = serde_json::from_value(json_value)?;
    if play_response.code != 200 {
        return Err(anyhow!("create playlist failed."));
    }
    Ok(play_response.playlist)
}

#[derive(serde::Serialize)]
struct DeletePlaylist {
    id: i64,
    pid: i64
}


pub async fn delete_playlist(client: &NeteaseClient, pid: i64) -> Result<()>{
    let params = DeletePlaylist {
        id: pid,
        pid,
    };
    let json_value = client.helper(API_URL, "playlist/delete", params, Method::Post).await?;
    let play_response: Status = serde_json::from_value(json_value)?;
    if play_response.code != 200 {
        return Err(anyhow!("delete playlist failed."));
    }
    Ok(())
}



