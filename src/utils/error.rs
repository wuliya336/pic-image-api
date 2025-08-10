use crate::utils::json::send_json_with_status;
use actix_web::{error::{InternalError, QueryPayloadError}, http::StatusCode, Error as HTTP_ERROR, HttpRequest};
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
    IoError(String),
    #[error("读取文件错误: {0}")]
    /// 读取文件错误
    ReadFileError(String),
    // #[error("其他错误: {0}")]
    // 其他错误
    // Other(String),
}

/// IO错误处理
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(format!("IO错误: {}", err))
    }
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
