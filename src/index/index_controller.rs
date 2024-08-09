use actix_web::{error, http::header, get, HttpResponse};
use log;
use crate::templates::template::{render_temp_controller, TEMPLATES};

use super::super::player::player_service::get_player_info;


#[get("/")]
async fn index_controller() -> actix_web::Result<HttpResponse, actix_web::Error> {
    let game = match get_player_info().await{
        Ok(x) => x,
        Err(err) => {
            log::error!("{err:?}");
            return Err(error::ErrorInternalServerError(err));
        }
    };
    for pat in TEMPLATES.get_template_names() {
        println!("{}", pat);
    }
    let result = render_temp_controller(&TEMPLATES, "index.html", &game.p.temp())?;
    Ok(HttpResponse::Ok().insert_header(header::ContentType::html()).body(result))
}


