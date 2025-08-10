use crate::utils::{error::ErrorResponse, image::{get_random_image, send_image}, json::send_json};
use actix_web::{get, http::StatusCode, web, Responder};
use std::path::Path;

#[get("/image/{image_name}")]
async fn pic_image(image_name: web::Path<String>) -> impl Responder{
    let image_name = image_name.into_inner();
    let image_dir = format!("data/{}", image_name);
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