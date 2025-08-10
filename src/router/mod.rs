mod api;

use actix_web::web;
use crate::router::api::register_api_routes;

pub fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(register_api_routes));
}