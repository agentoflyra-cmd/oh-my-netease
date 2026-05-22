use anyhow::{anyhow, Result};
use reqwest::{Client, ClientBuilder, Url, header::{CONNECTION, COOKIE, HOST, HeaderValue, REFERER, USER_AGENT}};

use crate::api::dto::{Status, UserConfig, UserLevel, UserLevelData };

#[allow(dead_code)]
const HOST_URL: &str = "music.163.com";
#[allow(dead_code)]
const BASE_URL: &str = "http://music.163.com";
#[allow(dead_code)]
const API_URL: &str = "http://music.163.com/api/";
#[allow(dead_code)]
const WE_API: &str = "http://music.163.com/weapi/";
#[allow(dead_code)]
const WE_API_V1: &str = "http://music.163.com/weapi/v1/";
#[allow(dead_code)]
const WE_API_V3: &str = "http://music.163.com/weapi/v3/";
#[allow(dead_code)]
const V_API: &str = "http://music.163.com/eapi/";
#[allow(dead_code)]
const AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0";

enum Method {
    Get,
    Post,
}

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

        Ok(Self { http, selected_user })
    }

    async fn helper(&self, target_host: &str, input: &str, params: &[(&str, &str)], method: Method) -> Result<serde_json::Value> {
        match method {
            Method::Get => {
                // let netease_client = NeteaseClient::build(1)?;
                // let http = self.http;
                let url = Url::parse(target_host)?.join(input)?;
                let resp = self.http.get(url).send().await?;
                let json_value:serde_json::Value = resp.json().await?;
                Ok(json_value)
            }
            Method::Post => {
                // let netease_client = NeteaseClient::build(1)?;
                // let http = self.http;
                let url = Url::parse(target_host)?.join(input)?;
                let resp = self.http.post(url).form(params).send().await?;
                let json_value:serde_json::Value = resp.json().await?;
                Ok(json_value)
            }
        }
    }

   pub async fn check_cookies(&self) -> Result<()> {
        let value = self.helper(API_URL, "push/init", &[], Method::Post).await?;
        let status: Status = serde_json::from_value(value)?;
        if status.code != 200 {
            return Err(anyhow!("client: check_cookies: Cookie expired or login failed."));
        }
        Ok(())
    }

    pub async fn user_level(&self) -> Result<UserLevelData> {
        let value = self.helper(API_URL, "user/level", &[], Method::Post).await?;
        let user_level: UserLevel = serde_json::from_value(value)?;
        if user_level.code != 200 {
            return Err(anyhow!("client: user_level: get user_level failed!"));
        }
        Ok(user_level.data)
    }

    // pub async fn user_profile(&self) -> Result<>
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use anyhow::{anyhow, Result};
    use reqwest::Url;
use serde_json::json;
    use crate::api::weapi;
    

    use crate::api::client::{API_URL, NeteaseClient, WE_API};
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

    #[tokio::test]
    pub async fn test_get_level() -> Result<()> {
        let netease_client = NeteaseClient::build(1)?;
        let http = netease_client.http;
        let url = Url::parse(API_URL)?.join("user/level")?;
        let resp = http.post(url).body("").send().await?;
        
        let json_value:serde_json::Value = resp.json().await?;
        println!("{}", serde_json::to_string_pretty(&json_value)?);
        Err(anyhow!("show for test!"))
        // Ok(())
    }

    #[tokio::test]
    pub async fn test_user_profile() -> Result<()> {
        let netease_client = NeteaseClient::build(1)?;
        let http = &netease_client.http;
        let url = Url::parse(WE_API)?.join("share/userprofile/info")?;
        let user_id = netease_client.user_level().await?.user_id;

        let mut params = HashMap::new();
        params.insert("userId", user_id);
        let encrypt = weapi::encrypt_request(json!(params))?;
        
        let resp = http.post(url).form(&encrypt).send().await?;
        
        // let status = resp.status();
        // let text = resp.text().await?;
        
        // println!("status: {:?}", resp.status());
        // println!("headers: {:?}", resp.headers());
        // println!("body: {:?}", resp.text().await?);
        let json_value:serde_json::Value = resp.json().await?;
        println!("{}", serde_json::to_string_pretty(&json_value)?);

        
        Err(anyhow!("show for test"))
    }

}
