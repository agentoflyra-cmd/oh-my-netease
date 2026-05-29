use anyhow::{anyhow, Result};
use reqwest::{Client, ClientBuilder, Url, header::{CONNECTION, COOKIE, HOST, HeaderValue, REFERER, USER_AGENT}};
use serde::Serialize;

use crate::api::{AGENT, BASE_URL, HOST_URL, dto::UserConfig, eapi, weapi};

pub enum Method {
    Get,
    Post,
}

#[derive(Debug, Default, Serialize)]
pub struct EmptyForm;

pub struct NeteaseClient {
    http: Client,
    selected_user: String
}

impl NeteaseClient {
    pub fn build(user_choice: usize) -> Result<Self> {
        let users = UserConfig::new()?;
        let selected_user = users.get_user_cookie(user_choice)?.to_string();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(AGENT));
        headers.insert(REFERER, HeaderValue::from_static(BASE_URL));
        headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
        headers.insert(HOST, HeaderValue::from_static(HOST_URL));
        headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));
        

        if user_choice > users.users {
            return Err(anyhow!(
                "login: user choice should not more than the number of cookies."
            ));
        }
        let cookie_value = format!("{}; appver=1.2.1; os=osx", users.cookies.get(user_choice - 1).unwrap());
        headers.insert(COOKIE, HeaderValue::from_str(&cookie_value)?);

        let http = ClientBuilder::new()
            .default_headers(headers)
            .build()?;

        Ok(Self { http, selected_user})
    }

    pub(crate) fn csrf_token(&self) -> Option<&str> {
        self.selected_user
            .split(';')
            .map(str::trim)
            .find_map(|pair| pair.strip_prefix("__csrf="))
    }

    pub(crate) async fn helper(&self, target_host: &str, input: &str, params: impl Serialize, method: Method) -> Result<serde_json::Value> {
        match method {
            Method::Get => {
                // let netease_client = NeteaseClient::build(1)?;
                // let http = self.http;
                let url = Url::parse(target_host)?.join(input)?;
                let resp = self.http.get(url).query(&params).send().await?;
                let json_value:serde_json::Value = resp.json().await?;
                Ok(json_value)
            }
            Method::Post => {
                // let netease_client = NeteaseClient::build(1)?;
                // let http = self.http;
                let url = Url::parse(target_host)?.join(input)?;
                let resp = self.http.post(url).form(&params).send().await?;
                let json_value:serde_json::Value = resp.json().await?;
                Ok(json_value)
            }
        }
    }

    pub(crate) async fn post_weapi(&self, target_host: &str, input: &str, params: impl Serialize) -> Result<serde_json::Value> {
        let url = Url::parse(target_host)?.join(input)?;
        let encrypt = weapi::encrypt_request(params)?;
        let resp = self.http.post(url).form(&encrypt).send().await?;
        let json_value: serde_json::Value = resp.json().await?;
        Ok(json_value)
    }

    pub(crate) async fn post_eapi(&self, target_host: &str, input: &str, params: impl Serialize) -> Result<serde_json::Value> {
        let url = Url::parse(target_host)?.join(input)?;
        let eapi_path = format!("/api/{}", input.trim_start_matches('/'));
        let encrypt = eapi::eapi_encrypt(eapi_path.as_bytes(), params)?;
        let resp = self.http.post(url).form(&encrypt).send().await?;
        let json_value: serde_json::Value = resp.json().await?;
        Ok(json_value)
    }

}

#[cfg(test)]
mod test {
use anyhow::{Result, anyhow};
    use reqwest::Url;
    use crate::api::{
        API_URL, V_API, client::NeteaseClient, endpoints::{artist, auto, playlist::{self, delete_playlist}, search, user}
    };

    #[ignore]
    #[tokio::test]
    pub async fn test_login() -> Result<()> {
        let netease_client = NeteaseClient::build(1)?;
        let http = netease_client.http;
        let url = Url::parse(API_URL)?.join("push/init")?;
        let resp = http.post(url).body("").send().await?;
        let json_value:serde_json::Value = resp.json().await?;
        println!("{}", serde_json::to_string_pretty(&json_value)?);
        
        // Err(anyhow!("show for test!"))
        Ok(())
        
    }

    #[ignore]
    #[tokio::test]
    pub async fn test_get_level() -> Result<()> {
        let netease_client = NeteaseClient::build(1)?;
        let level = user::user_level(&netease_client).await?;
        println!("{}", serde_json::to_string_pretty(&level)?);
        // Err(anyhow!("show for test!"))
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    pub async fn test_user_profile() -> Result<()> {
        let netease_client = NeteaseClient::build(1)?;
        let user_level = user::user_level(&netease_client).await?;
        let user_profile = user::user_profile(&netease_client, user_level.user_id).await?;
        println!("{}", serde_json::to_string_pretty(&user_profile)?);
        Err(anyhow!("show for test"))
        // Ok(())
    }

    #[ignore]
    #[tokio::test]
    pub async fn test_user_playlists() -> Result<()> {
        let netease_client = NeteaseClient::build(1)?;
        let user_playlists = playlist::user_playlists(&netease_client, 1929567926, 0, 200).await?;
        println!("{}", serde_json::to_string_pretty(&user_playlists)?);
        Err(anyhow!("show for test"))
        // Ok(())
    }

    #[ignore]
    #[tokio::test]
    pub async fn test_djradio() -> Result<()> {
        let netease_client = NeteaseClient::build(1)?;
        let json_value = netease_client
            .post_eapi(V_API, "djradio/subed/v1", serde_json::json!({
                "limit": 100,
                "time": 0,
                "needFee": false,
            }))
            .await?;
        println!("{}", serde_json::to_string_pretty(&json_value)?);
        Err(anyhow!("show for test"))
        // Ok(())
    }

    #[ignore]
    #[tokio::test]
    pub async fn test_check_cookies() -> Result<()> {
        let netease_client = NeteaseClient::build(1)?;
        auto::check_cookies(&netease_client).await?;
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    pub async fn test_user_favorite_albums() -> Result<()> {
        let netease_client = NeteaseClient::build(1)?;
        // let params = json!({
        //     "offset": 0,
        //     "limit": 30,
        //     "total": true,
        //     "csrf_token": netease_client.csrf_token().unwrap_or_default(),
        // });
        let json_value = playlist::user_favorite_albums(&netease_client, 0, 30).await?;
        println!("{}", serde_json::to_string_pretty(&json_value)?);
        Ok(())
        // Err(anyhow!("show for test"))
    }

    #[ignore]
    #[tokio::test]
    pub async fn test_user_favorite_artists() -> Result<()> {
        let netease = NeteaseClient::build(1)?;
        let json_value = playlist::user_favorite_artists(&netease, 0, 30).await?;
        println!("{}", serde_json::to_string_pretty(&json_value)?);
        // Err(anyhow!("show for test"))
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    pub async fn test_search() -> Result<()> {
        let client = NeteaseClient::build(1)?;
        let songs = search::search_songs(&client, "test", 0, 1).await?;
        println!("{}", serde_json::to_string_pretty(&songs)?);
        let artists = search::search_artists(&client, "test", 0, 1).await?;
        println!("{}", serde_json::to_string_pretty(&artists)?);
        let albums = search::search_albums(&client, "test", 0, 1).await?;
        println!("{}", serde_json::to_string_pretty(&albums)?);
        let users = search::search_users(&client, "test", 0, 1).await?;
        println!("{}", serde_json::to_string_pretty(&users)?);
        let playlists = search::search_playlists(&client, "test", 0, 1).await?;
        println!("{}", serde_json::to_string_pretty(&playlists)?);
        // Err(anyhow!("show for test"))
        Ok(())
    }

    #[ignore]
    #[tokio::test]
    pub async fn test_detail_v3() -> Result<()> {
        let client = NeteaseClient::build(1)?;
        let detail = playlist::detail_v3(&client, 7580255366, 0, 10).await?;
        println!("{}", serde_json::to_string_pretty(&detail)?);
        Ok(())
    }

    #[derive(serde::Serialize)]
    struct UpdatePlaylistNameParams<'a> {
        id: i64,
        name: &'a str,
    }
    
    #[ignore]
    #[tokio::test]
    pub async fn test_new_playlist() -> Result<()> {
        let client = NeteaseClient::build(1)?;
        let params = UpdatePlaylistNameParams {
            // user id
            id: 1929567926,
            name: "playlist for test",
        };
        let json_value = client.helper(API_URL, "playlist/create", params, crate::api::client::Method::Post).await?;
        println!("{}", serde_json::to_string_pretty(&json_value)?);
        Err(anyhow!("show for test"))
    }

    #[ignore]
    #[tokio::test]
    pub async fn test_update_playlist_name() -> Result<()> {
        let client = NeteaseClient::build(1)?;
        let params = UpdatePlaylistNameParams {
            // playlist id
            id: 17996450646,
            name: "update playlist name for test"
        };
        let json_value = client.helper(API_URL, "playlist/update/name", params, crate::api::client::Method::Post).await?;
        println!("{}", serde_json::to_string_pretty(&json_value)?);
        Err(anyhow!("show for test"))
    }

    #[derive(serde::Serialize)]
    struct DeletePlaylist {
        id: i64,
        pid: i64
    }

    #[ignore]
    #[tokio::test]
    pub async fn test_delete_playlist() -> Result<()> {
        let client = NeteaseClient::build(1)?;
        delete_playlist(&client, 17996450646).await
    }

    #[ignore]
    #[tokio::test]
    pub async fn test_artist_info() -> Result<()> {
        let client = NeteaseClient::build(1)?;
        let artist_info = artist::artist_info(&client, 8234).await?;
        println!("{}", serde_json::to_string_pretty(&artist_info)?);
        Ok(())
    }

}
