mod api;
pub(crate) mod admin;

use crate::router::api::register_api_routes;
use actix_web::web;

pub(crate) fn register_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(register_api_routes));
    cfg.service(web::scope("/admin").configure(admin::register_admin_routes));
}
