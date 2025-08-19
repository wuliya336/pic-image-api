use crate::utils::{
    error::Error,
    file::{get_random_file_path, send_file},
};
use actix_web::HttpResponse;
use std::path::Path;
use tokio::fs;

/// 发送图片文件
pub fn send_image(data: Vec<u8>) -> HttpResponse {
    send_file(data)
}

/// 随机图片结构体
#[allow(dead_code)]
pub struct RandomImage {
    /// 图片路径
    pub path: String,
    /// 图片数据
    pub data: Vec<u8>,
}
/// 从指定目录中随机获取一张图片的内容
///
/// 该函数会从给定目录中随机选择一张图片文件，并返回其二进制内容。
/// 支持的图片格式包括：PNG、JPG、JPEG、WEBP。
///
/// # 参数
/// * `path` - 要搜索图片的目录路径
///
/// # 返回值
/// 成功时返回随机选择的图片文件的二进制内容，失败时返回错误
///
/// # 错误
/// * 当目录不存在或无法访问时
/// * 当目录中没有支持的图片格式文件时
/// * 当读取选中的图片文件失败时
pub async fn get_random_image(path: &Path) -> Result<RandomImage, Error> {
    let image_path = get_random_file_path(path, Some(vec!["png", "jpg", "jpeg", "webp"])).await?;
    let image_data = fs::read(&image_path)
        .await
        .map_err(|err| Error::Io(err))?;
    Ok(RandomImage {
        path: image_path,
        data: image_data,
    })
}
