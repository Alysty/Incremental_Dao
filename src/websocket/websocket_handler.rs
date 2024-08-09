// use actix::{fut, ActorContext};
// // use crate::messages::{Disconnect, Connect, WsConnMessage, ClientActorMessage}; //We'll be writing this later
// // use crate::lobby::Lobby; // as well as this
// use actix::{Actor, Addr, Running, StreamHandler, WrapFuture, ActorFuture, ContextFutureSpawner};
// use actix::{AsyncContext, Handler};
// use actix_web_actors::ws;
// use std::time::{Duration, Instant};
//
// const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
// const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);
//
// /// Define HTTP actor
// struct WsConn {
//     hb: Instant,
// }
// impl WsConn {
//     pub fn new() -> WsConn {
//         WsConn{
//             hb: Instant::now(),
//         }
//     }
//     fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
//         ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
//             if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
//                 println!("Disconnecting failed heartbeat");
//                 ctx.stop();
//                 return;
//             }
//             ctx.ping(b"PING");
//         });
//     }
// }
//
// impl Actor for WsConn {
//     type Context = ws::WebsocketContext<Self>;
//     fn started(&mut self, ctx: &mut Self::Context) {
//         self.hb(ctx);
//         // let addr = ctx.address();
//     }
//     fn stopping(&mut self, _: &mut Self::Context) -> Running {
//         Running::Stop
//     }
//    
// }
// impl Handler<WsMessage> for WsConn{
//     type Result = ();
//     fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
//         
//     }
// }
//
//
//
// impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
//     fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
//         match msg {
//             Ok(ws::Message::Ping(msg)) => {
//                 self.hb = Instant::now();
//                 ctx.pong(&msg);
//             }
//             Ok(ws::Message::Pong(_)) => {
//                 self.hb = Instant::now();
//             }
//             Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
//             Ok(ws::Message::Close(reason)) => {
//                 ctx.close(reason);
//                 ctx.stop();
//             }
//             Ok(ws::Message::Continuation(_)) => {
//                 ctx.stop();
//             }
//             Ok(ws::Message::Nop) => (),
//             Ok(ws::Message::Text(_s)) => (),
//             Err(e) => panic!("{}", e),
//
//         }
//     }
// }
//
//
// // async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
// //     let resp = ws::start(WsConn {}, &req, stream);
// //     println!("{:?}", resp);
// //     resp
// // }
