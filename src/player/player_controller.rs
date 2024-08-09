use actix_web::{error, http::header, post, web, HttpResponse, Scope};
use log;
use crate::templates::template::{render_temp_controller, TEMPLATES};

use super::player_service::tick_update_qi;

#[post("/get_info")]
async fn get_info(
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().finish())
}
#[post("/tick_update_qi")]
async fn tick_update_qi_controller(
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    let game = match tick_update_qi().await {
        Ok(x) => x,
        Err(err) => {
            log::error!("{err:?}");
            return Err(error::ErrorInternalServerError(err));
        }
    };
    let result = render_temp_controller(&TEMPLATES, "play_info.html", &game.p.temp())?;
    Ok(HttpResponse::Ok().insert_header(header::ContentType::html()).body(result))
}


pub fn get_route_config() -> Scope {
    web::scope("player").service(get_info);
    web::scope("player").service(tick_update_qi_controller)
}
