use actix_web::{get, Responder};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use crate::utils::config::Config;
use crate::utils::json::send_json;

#[derive(Debug, Serialize, Deserialize)]
struct InfosResponse {
    code: u16,
    message: String,
    data: Option<Vec<Info>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    name: String,
    description: String,
    folder_name: String,
}
#[get("/infos")]
async fn infos() -> impl Responder {
    let config = Config::api();
    let infos = config.into_iter().map(|info| Info {
        name: info.name,
        description: info.description,
        folder_name: info.folder_name,
    }).collect::<Vec<Info>>();
    let res = InfosResponse {
        code: StatusCode::OK.as_u16(),
        message: "success".to_string(),
        data: Some(infos),
    };
    send_json(&res)
}