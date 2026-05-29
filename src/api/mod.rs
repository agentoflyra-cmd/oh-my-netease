pub mod client;
pub mod dto;
pub mod eapi;
pub mod endpoints;
pub mod weapi;

#[allow(dead_code)]
pub(crate) const HOST_URL: &str = "music.163.com";
#[allow(dead_code)]
pub(crate) const BASE_URL: &str = "http://music.163.com";
#[allow(dead_code)]
pub(crate) const API_URL: &str = "http://music.163.com/api/";
#[allow(dead_code)]
pub(crate) const WE_API: &str = "http://music.163.com/weapi/";
#[allow(dead_code)]
pub(crate) const WE_API_V1: &str = "http://music.163.com/weapi/v1/";
#[allow(dead_code)]
pub(crate) const WE_API_V3: &str = "http://music.163.com/weapi/v3/";
#[allow(dead_code)]
pub(crate) const V_API: &str = "https://music.163.com/eapi/";
#[allow(dead_code)]
pub(crate) const AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0";
