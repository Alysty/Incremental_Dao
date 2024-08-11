use actix_web::{error, get, http::header, post, web, HttpRequest, HttpResponse, Scope};
use actix_web_actors::ws;
use super::websocket_handler::WsConn;
use log;
// use crate::templates::template::{render_temp_controller, TEMPLATES};

#[get("/")]
async fn websocket(req: HttpRequest, stream : web::Payload
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    ws::start(WsConn::new(), &req, stream)
}

#[post("/test")]
async fn websocket_test(req: HttpRequest, stream : web::Payload
) -> actix_web::Result<HttpResponse, actix_web::Error> {
    ws::start(WsConn::new(), &req, stream)
}

pub fn get_route_config() -> Scope {
    web::scope("websocket").service(websocket_test);
    web::scope("websocket").service(websocket)
}
