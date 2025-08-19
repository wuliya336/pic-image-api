use std::path::Path;
use serde::{Deserialize, Serialize};
use super::file::read_config;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
}
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "星柠图片API".to_string(),
            host: "127.0.0.1".to_string(),
            port: 33720,
        }
    }
}
impl ServerConfig {
    pub fn get() -> Self {
        read_config::<ServerConfig>(Path::new("config"), "server")
            .unwrap_or_else(|_| ServerConfig::default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInfo {
    pub name: String,
    pub description: String,
    pub folder_name: String,
}
impl Default for ApiInfo {
    fn default() -> Self {
        Self {
            name: "loli".to_string(),
            description: "萝莉图描述".to_string(),
            folder_name: "loli".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ApiConfig {
    api: Vec<ApiInfo>,
}

impl ApiConfig {
    pub fn get() -> Vec<ApiInfo> {
        read_config::<ApiConfig>(Path::new("config"), "api")
            .map(|wrapper| wrapper.api)
            .unwrap_or_else(|_| vec![ApiInfo::default()])
    }
}

pub struct Config;

impl Config {
    pub fn server() -> ServerConfig {
        ServerConfig::get()
    }

    pub fn api() -> Vec<ApiInfo> {
        ApiConfig::get()
    }
}
