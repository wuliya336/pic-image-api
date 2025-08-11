use actix_web::{
    middleware::{NormalizePath, TrailingSlash}, web,
    App,
    HttpServer,
};
use dotenv::dotenv;
use std::env;

mod middleware;
mod utils;
mod router;

use middleware::logger::log_init;
use utils::error::query_error_handler;
use crate::router::register_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // 加载环境变量
    log_init(); // 初始化日志

    log::info!("欢迎使用PIC-IMAGE_API");

    let host = env::var("HTTP_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("HTTP_PORT").unwrap_or("33720".to_string());
    let address = format!("{}:{}", host, port);

    log::info!("启动服务: {}", address);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::access_log::AccessLog)
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(web::QueryConfig::default().error_handler(query_error_handler))
            .configure(register_routes)
    })
    .bind(address)?
    .run()
    .await
}
