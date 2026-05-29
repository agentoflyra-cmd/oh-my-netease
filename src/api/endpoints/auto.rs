use anyhow::{anyhow, Result};

use crate::api::{API_URL, client::{EmptyForm, NeteaseClient}, dto::Status};

pub async fn check_cookies(client: &NeteaseClient) -> Result<()> {
    let value = client
        .helper(
            API_URL,
            "push/init",
            EmptyForm,
            crate::api::client::Method::Post,
        )
        .await?;
    let status: Status = serde_json::from_value(value)?;
    if status.code != 200 {
        return Err(anyhow!(
            "client: check_cookies: Cookie expired or login failed."
        ));
    }
    Ok(())
}
