use std::collections::HashMap;

use crate::api::{API_URL, WE_API};
use crate::api::client::{EmptyForm, Method, NeteaseClient};
use crate::api::dto::{UserLevelData, UserLevel, UserProfile};
use anyhow::{Result, anyhow};
use serde_json::json;
pub async fn user_level(client: &NeteaseClient) -> Result<UserLevelData> {
    let value = client
        .helper(
            API_URL,
            "user/level",
            EmptyForm,
            Method::Post,
        )
        .await?;
    let user_level: UserLevel = serde_json::from_value(value)?;
    if user_level.code != 200 {
        return Err(anyhow!("client: user_level: get user_level failed!"));
    }
    Ok(user_level.data)
}

pub async fn user_profile(client: &NeteaseClient, user_id: i64) -> Result<UserProfile> {
    let mut params = HashMap::new();
    params.insert("userId", user_id);
    let value = client.post_weapi(WE_API, "share/userprofile/info", json!(params)).await?;
    let user_profile: UserProfile = serde_json::from_value(value)?;
    if user_profile.code != 200 {
        return Err(anyhow!("client: user_profile: get user_profile failed!"));
    }
    Ok(user_profile)
}
