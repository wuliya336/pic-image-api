use crate::utils::json::send_json_with_status;
use actix_web::{
    Error as HTTP_ERROR, HttpRequest,
    error::{InternalError, QueryPayloadError},
    http::StatusCode,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
/// 错误响应结构体
pub struct ErrorResponse {
    /// 状态码
    pub code: u16,
    /// 响应信息
    pub message: String,
    /// 响应数据
    pub data: Option<String>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO错误: {0}")]
    /// IO错误
    Io(#[from] std::io::Error),
    #[error("JSON解析错误: {0}")]
    /// JSON解析错误
    Json(#[from] serde_json::Error),
    #[error("TOML解析错误: {0}")]
    Toml(#[from] toml::de::Error),
    // #[error("其他错误: {0}")]
    // 其他错误
    // Other(String),
}

/// 查询参数错误处理
pub fn query_error_handler(err: QueryPayloadError, _req: &HttpRequest) -> HTTP_ERROR {
    log::info!("query error: {:?}", err);
    let message = match &err {
        QueryPayloadError::Deserialize(e) => {
            format!("参数错误: {}", e)
        }
        _ => format!("参数错误: {}", err),
    };
    let error_response = ErrorResponse {
        code: StatusCode::BAD_REQUEST.as_u16(),
        message,
        data: None,
    };

    let http_response = send_json_with_status(&error_response, StatusCode::BAD_REQUEST);
    InternalError::from_response(err, http_response).into()
}
