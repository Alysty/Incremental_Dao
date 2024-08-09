use actix::{
    prelude::Message, Actor, Addr,
    Context, Handler, 
};
use crate::{gen_errors::errors::GeneralReportError, player::player_structs::Player};
use lazy_static::lazy_static;
lazy_static!{
    pub static ref GAME_ACTOR: Addr<Game> = Game::start_default();
}
#[derive(Debug, Clone)]
pub struct Game {
    pub p : Player
}

impl Actor for Game {
    type Context = Context<Self>;
}

impl Default for Game {
    fn default() -> Self {
        Game {p:Player { name: "".to_string(), qi: 0 }}
    }
}

impl Message for Game {
    type Result = Result<Game, GeneralReportError>;
}

#[derive(Debug, Message)]
#[rtype(result= "Result<Game, GeneralReportError>")]
pub struct Tick {
    pub message: String
}

impl Handler<Tick> for Game {
    type Result = Result<Game, GeneralReportError>;
    fn handle(&mut self, _msg: Tick, _ctx: &mut Self::Context) -> Self::Result {
        self.p.qi += 1;
        Ok(self.clone())
    }
}


#[derive(Debug, Message)]
#[rtype(result= "Result<Game, GeneralReportError>")]
pub struct GetGame;
impl Handler<GetGame> for Game {
    type Result = Result<Game, GeneralReportError>;
    fn handle(&mut self, _msg: GetGame, _ctx: &mut Self::Context) -> Self::Result {
        Ok(self.clone())
    }
}
