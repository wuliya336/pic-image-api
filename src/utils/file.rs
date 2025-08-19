use crate::utils::error::Error;
use actix_web::HttpResponse;
use actix_web::http::StatusCode;
#[cfg(not(debug_assertions))]
use mime_infer;
use rand::{Rng, rng};
use std::path::Path;
use serde::de::DeserializeOwned;
use toml::from_str;
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

pub fn read_config<D>(path: &Path, name: &str) -> Result<D, Error>
where
    D: DeserializeOwned,
{
    let full_path = path.join(format!("{}.toml", name));
    let config_str = std::fs::read_to_string(full_path).map_err(Error::Io)?;
    let config: D = from_str(&config_str).map_err(Error::Toml)?;
    Ok(config)
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
        .map_err(Error::Io)?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(Error::Io)?
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
        return Err(Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "目录中没有文件")));
    }

    let mut rng = rng();
    let random_index = rng.random_range(0..valid_files.len());
    Ok(valid_files[random_index].clone())
}

/// 获取文件的 MIME 类型
///
/// 该函数通过文件内容来推断 MIME 类型，
/// 如果推断失败，则返回默认的 MIME 类型。
///
/// # 参数
/// * `data` - 文件内容的字节数据，用于内容类型推断
///
/// # 返回值
/// 返回推断出的 MIME 类型字符串，如果推断失败则返回 "application/octet-stream"
pub fn get_mime_type(data: &[u8]) -> String {
    infer::get(data)
        .map(|kind| kind.mime_type().to_string())
        .unwrap_or("application/octet-stream".to_string())
}

#[cfg(not(debug_assertions))]
/// 通过文件扩展名获取 MIME 类型
///
/// 该函数通过文件路径的扩展名来推断 MIME 类型，
/// 如果推断失败，则返回默认的 MIME 类型。
///
/// # 参数
/// * [path] - 文件路径，用于提取扩展名
///
/// # 返回值
/// 返回推断出的 MIME 类型字符串，如果推断失败则返回 "application/octet-stream"
pub fn get_mime_type_from_extension(path: &str) -> String {
    mime_infer::from_path(path)
        .first_or_octet_stream()
        .to_string()
}
