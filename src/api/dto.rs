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

#[cfg(test)]
mod test {
    use crate::api::dto::get_cookies;
    use anyhow::{anyhow, Result};

    #[test]
    fn test_read_db() -> Result<()> {
        let results = get_cookies()?;
        dbg!(&results);
        assert!(!results.is_empty());
        Err(anyhow!("err for test"))
    }
}
