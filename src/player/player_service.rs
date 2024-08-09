use error_stack::Result;
use crate::game::game_actor::{Game, GetGame, Tick, GAME_ACTOR};

use super::super::gen_errors::errors::GeneralReportError;

pub async fn tick_update_qi() -> Result<Game, GeneralReportError> {
    let game = GAME_ACTOR.send(Tick { message: "Test".to_string() }).await
    .map_err(GeneralReportError::from_err)?
    .map_err(GeneralReportError::from_err)?;
    Ok(game)
}

pub async fn get_player_info() -> Result<Game, GeneralReportError> {
    let game = GAME_ACTOR.send(GetGame {}).await
    .map_err(GeneralReportError::from_err)?
    .map_err(GeneralReportError::from_err)?;
    Ok(game)
}

