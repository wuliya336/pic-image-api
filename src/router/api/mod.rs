mod image;

use actix_web::web;

pub(crate) fn register_api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(image::pic_image);
}