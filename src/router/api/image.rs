use crate::utils::{
    error::ErrorResponse,
    image::{get_random_image, send_image},
    json::send_json,
};
use actix_web::{Responder, http::StatusCode};
use std::path::Path;

pub async fn pic_image(folder_name: String) -> impl Responder {
    let image_dir = format!("data/{}", folder_name);
    let image = match get_random_image(Path::new(&image_dir)).await {
        Ok(image) => image,
        Err(err) => {
            log::error!("{}", err);
            let response = ErrorResponse {
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "服务器内部错误".to_string(),
                data: None,
            };
            return send_json(&response);
        }
    };

    send_image(image.data)
}
