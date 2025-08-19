use crate::router::{admin, register_routes};
#[cfg(not(debug_assertions))]
use crate::utils::file::get_mime_type_from_extension;
use actix_web::{
    App, HttpServer,
    middleware::{NormalizePath, TrailingSlash},
    web,
};
use middleware::logger::log_init;
#[cfg(debug_assertions)]
use actix_files::Files;
#[cfg(not(debug_assertions))]
use rust_embed::RustEmbed;
#[cfg(debug_assertions)]
use std::{process, path::Path};
use utils::{error::query_error_handler, config::Config};
#[cfg(not(debug_assertions))]
#[derive(RustEmbed)]
#[folder = "web/dist"]
struct Assets;

mod middleware;
mod router;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    admin::init_start_time();
    log_init(); // 初始化日志
    let server = Config::server();
    log::info!("欢迎使用PIC-IMAGE_API");

    let host =  server.host;
    let port = server.port;
    let address = format!("{}:{}", host, port);

    log::info!("启动服务: {}", address);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::access_log::AccessLog)
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .app_data(web::QueryConfig::default().error_handler(query_error_handler))
            .configure(register_routes)
            .configure(register_static)
    })
    .bind(address)?
    .run()
    .await
}

pub fn register_static(cfg: &mut web::ServiceConfig) {
    #[cfg(debug_assertions)]
    {
        let source = "./web/dist";
        if !Path::exists(Path::new(source)){
            log::error!("{} 目录不存在", source);
            process::exit(1);
        }
        cfg.service(Files::new("/", source).index_file("index.html"));
    }
    #[cfg(not(debug_assertions))]
    cfg.route("/{path:.*}", web::get().to(index_handler));
}





#[cfg(not(debug_assertions))]
async fn index_handler(req: actix_web::HttpRequest) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    let path = req.path();
    let asset_path = if path == "/" || path.is_empty() {
        "index.html"
    } else {
        path.trim_start_matches('/')
    };

    match Assets::get(asset_path) {
        Some(content) => {
            let mime_type = get_mime_type_from_extension(asset_path);
            HttpResponse::Ok()
                .content_type(mime_type)
                .body(content.data.into_owned())
        }
        None => {
            if let Some(content) = Assets::get("index.html") {
                HttpResponse::Ok()
                    .content_type("text/html")
                    .body(content.data.into_owned())
            } else {
                HttpResponse::NotFound().body("404 Not Found")
            }
        }
    }
}
