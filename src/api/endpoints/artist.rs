use anyhow::{anyhow, Result};

use crate::api::{
    API_URL,
    client::{Method, NeteaseClient},
    dto::ArtistInfoResponse,
};

pub async fn artist_info(client: &NeteaseClient, artist_id: i64) -> Result<ArtistInfoResponse> {
    let value = client
        .helper(
            API_URL,
            &format!("artist/{artist_id}"),
            (),
            Method::Get,
        )
        .await?;
    let artist_info: ArtistInfoResponse = serde_json::from_value(value)?;
    if artist_info.code != 200 {
        return Err(anyhow!("client: artist_info: get artist info failed!"));
    }
    Ok(artist_info)
}
