use crate::utils::{config::Config, json::send_json, version::VERSION};
use actix_web::{get, Responder, http::StatusCode};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static START_TIME: OnceLock<i64> = OnceLock::new();
#[derive(Debug, Serialize, Deserialize)]
struct StatusInfo {
    app_name: String,
    version: String,
    uptime: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct StatusResponse {
    code: u16,
    message: String,
    data: Option<StatusInfo>,
}
#[get("/status")]
async fn status() -> impl Responder {
    let server = Config::server();

    let now = Utc::now().timestamp();
    let uptime = now - START_TIME.get().unwrap();
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    let seconds = uptime % 60;

    let mut uptime_str = String::new();

    if days > 0 {
        uptime_str.push_str(&format!("{} 天 ", days));
    }
    if hours > 0 {
        uptime_str.push_str(&format!("{} 小时 ", hours));
    }
    if minutes > 0 {
        uptime_str.push_str(&format!("{} 分钟 ", minutes));
    }
    uptime_str.push_str(&format!("{} 秒", seconds));
    let response = StatusResponse {
        code: StatusCode::OK.as_u16(),
        message: "OK".to_string(),
        data: Some(StatusInfo {
            app_name: server.name.to_string(),
            version: VERSION.to_string(),
            uptime: uptime_str,
        }),
    };
    send_json(&response)
}


pub fn init_start_time() {
    START_TIME.set(Utc::now().timestamp()).unwrap()
}