use crate::utils::error::ErrorResponse;
use actix_web::{HttpResponse, http::StatusCode};
use log::info;
use serde::Serialize;
use serde_json::{to_string, to_string_pretty};

/// 发送JSON响应
/// 优先发送美化后的JSON，失败则发送原始JSON
pub fn send_json<T>(data: &T) -> HttpResponse
where
    T: ?Sized + Serialize,
{
    send_json_with_status(data, StatusCode::OK)
}

/// 发送带指定状态码的JSON响应
pub fn send_json_with_status<T>(data: &T, status_code: StatusCode) -> HttpResponse
where
    T: ?Sized + Serialize,
{
    match to_string_pretty(data) {
        Ok(formatted_json) => HttpResponse::build(status_code)
            .content_type("application/json")
            .body(formatted_json),
        Err(_) => match to_string(data) {
            Ok(raw_json) => HttpResponse::build(status_code)
                .content_type("application/json")
                .body(raw_json),
            Err(e) => {
                info!("JSON序列化错误: {}", e);
                let response = ErrorResponse {
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    message: String::from("服务器内部错误"),
                    data: None,
                };
                HttpResponse::InternalServerError().json(response)
            }
        },
    }
}
