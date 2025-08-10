use crate::{utils::error::Error};
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use rand::{rng, Rng};
use std::path::Path;
use tokio::fs;

/// 发送文件内容
///
/// 该函数接收文件路径和文件数据，构建HTTP响应并返回文件内容。
/// 如果文件读取成功，返回状态码为200 OK的响应；
/// 如果读取失败，则记录错误日志并返回500错误响应。
///
/// # 参数
/// * `image_path` - 文件路径，用于确定MIME类型
/// * `data` - 包含文件数据的Result<Vec<u8>>，Ok表示读取成功，Err表示读取失败
///
/// # 返回值
/// 返回构建好的HTTP响应
pub fn send_file(data: Vec<u8>) -> HttpResponse {
    send_file_with_status(data, StatusCode::OK)
}

/// 发送文件内容并指定状态码
///
///
/// # 参数
/// * `image_path` - 文件路径，用于确定MIME类型
/// * `data` - 包含文件数据的Result<Vec<u8>>，Ok表示读取成功，Err表示读取失败
/// * `status_code` - 要返回的HTTP状态码
///
/// # 返回值
/// 返回构建好的HTTP响应
pub fn send_file_with_status(data: Vec<u8>, status_code: StatusCode) -> HttpResponse {
    let mime_type = get_mime_type(&data);
    HttpResponse::build(status_code)
        .content_type(mime_type.as_str())
        .body(data)
}

/// 随机从一个目录返回一个文件的路径
///
/// # 参数
/// * `path` - 目录路径
/// * `exp` - 可选的文件后缀列表，如果提供则只返回匹配后缀的文件
///
/// # 返回值
/// 返回一个随机文件的绝对路径
pub async fn get_random_file_path(path: &Path, exp: Option<Vec<&str>>) -> Result<String, Error> {
    let mut valid_files = Vec::new();

    let mut entries = fs::read_dir(path)
        .await
        .map_err(|err| Error::IoError(err.to_string()))?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|err| Error::IoError(err.to_string()))?
    {
        let path = entry.path();
        let metadata = fs::metadata(&path).await;

        if let Ok(metadata) = metadata {
            if metadata.is_file() {
                if let Some(ref ext_list) = exp {
                    if let Some(extension) = path.extension() {
                        let ext_str = extension.to_string_lossy().to_lowercase();
                        if !ext_list
                            .iter()
                            .any(|&ext| ext.trim_start_matches('.') == ext_str)
                        {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }

                if let Some(path_str) = path.to_str() {
                    valid_files.push(path_str.to_string());
                }
            }
        }
    }

    if valid_files.is_empty() {
        return Err(Error::IoError("目录中没有文件".to_string()));
    }

    let mut rng = rng();
    let random_index = rng.random_range(0..valid_files.len());
    Ok(valid_files[random_index].clone())
}


/// 获取文件的 MIME 类型
///
/// 该函数首先尝试通过文件内容来推断 MIME 类型，
/// 如果推断失败，则根据文件扩展名来猜测 MIME 类型。
///
/// # 参数
/// * `filename` - 文件名，用于根据扩展名猜测 MIME 类型
/// * `buffer` - 文件内容的字节缓冲区，用于内容类型推断
///
/// # 返回值
/// 返回推断出的 MIME 类型字符串
pub fn get_mime_type(data: &[u8]) -> String {
    let buffer = infer::get(data);
    let mime_type = match buffer {
        Some(kind) => kind.mime_type().to_string(),
        None => "application/octet-stream".to_string(),
    };
    mime_type
}