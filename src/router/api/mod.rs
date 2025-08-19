mod image;

use actix_web::web;
use crate::utils::config::Config;

pub(crate) fn register_api_routes(cfg: &mut web::ServiceConfig) {
    Config::api().into_iter().for_each(|api_config| {
        let folder_name = api_config.folder_name.clone();

        let image_scope = web::scope(folder_name.as_str())
            .route("", web::get().to({
                let folder_name = folder_name.clone();
                move || {
                    let folder_name = folder_name.clone();
                    async move {
                        image::pic_image(folder_name).await
                    }
                }
            }));

        cfg.service(image_scope);
    });
}