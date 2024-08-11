use actix::{
    Actor, ActorContext, Addr, AsyncContext, Handler, Message, ResponseFuture, Running, StreamHandler
};
use actix_web_actors::ws::{self, WebsocketContext};
use std::time::{Duration, Instant};
use crate::{game::game_actor::{Game, GetGame, GAME_ACTOR}, gen_errors::errors::GeneralReportError};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

static WEBSOCKET_CONN: Addr<WsConn> = Addr::new(WsConn::new());


/// Define HTTP actor
pub struct WsConn {
    hb: Instant,
}

impl WsConn {
    pub fn new() -> WsConn {
        WsConn { hb: Instant::now() }
    }
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                ctx.stop();
                return;
            }
            ctx.ping(b"PING");
        });
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        // let addr = ctx.address();
    }
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        Running::Stop
    }
}

#[derive(Debug, Message)]
#[rtype(result="Result<Game, GeneralReportError>")]
struct WsMessage {
    message: String
}


impl Handler<WsMessage> for WsConn {
    type Result = ResponseFuture<Result<Game, GeneralReportError>>;
    fn handle(&mut self, _msg: WsMessage, _ctx: &mut Self::Context) -> Self::Result {
        Box::pin(async move {
            // Some async computation
            let game = GAME_ACTOR.send(GetGame{})
                .await
                .map_err(GeneralReportError::from_err)?
                .map_err(GeneralReportError::from_err)?;
            Ok(game)
        })
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(ws::Message::Text(s)) => {
                println!(" Stream handle test {:?}", s)
            },
            Err(e) => panic!("{}", e),
        }
    }
}

// async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
//     let resp = ws::start(WsConn {}, &req, stream);
//     println!("{:?}", resp);
//     resp
// }
