mod status;
mod api;

use actix_web::web;
pub use status::init_start_time;

pub(crate) fn register_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(status::status);
    cfg.service(api::infos);
}