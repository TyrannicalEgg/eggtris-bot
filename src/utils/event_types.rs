use super::types::*;
use serde::{Deserialize, Serialize};
use serde_json::Number;


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerEvent {
    RoomData {payload: RoomDataType},
    Authenticated {payload: SessionIdType},
    Error {payload: ErrorType},
    PlayerJoined {payload: PlayerDataType},
    PlayerLeft {payload: SessionIdType},
    PlayerBanned {payload: BotInfoType},
    PlayerUnbanned {payload: BotInfoType},
    SettingsChanged {payload: RoomDataType},
    GameStarted {},
    RoundStarted {payload: RoundStartType},
    RequestMove {payload: RequestMoveType},
    Action {payload: ActionType},
    PlayerAction {payload: PlayerActionType},
    PlayerDamageReceived {payload: PlayerDamageReceivedType},
    RoundOver {payload: EndType},
    GameOver {payload: EndType},
    GameReset {payload: RoomDataType},
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomDataType {
    pub room_data: RoomData,
}

pub type ErrorType = String;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDataType {
    pub player_data: PlayerData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionIdType {
    pub session_id: SessionId,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BotInfoType {
    pub bot_info: BotInfo,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoundStartType {
    pub starts_at: Number,
    pub room_data: RoomData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestMoveType {
    pub game_state: GameState,
    pub players: Vec<PlayerData>
}

/// Represents a collection of moves to be sent to the server for the
/// current piece
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionType {
    pub commands: Vec<Command>
}

impl ActionType {
    pub fn new() -> ActionType {
        ActionType { commands: Vec::new() }
    }

    /// Adds a new command to the end of the vector
    /// containing the requested Commands
    pub fn push(&mut self, command: Command) -> &mut Self {
        self.commands.push(command);
        self
    }

    /// Appends all of the Commands in commands to the 
    /// end of the vector containing the requested Commands
    pub fn append(&mut self, commands: &mut Vec<Command>) -> &mut Self {
        self.commands.append(commands);
        self
    }
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerActionType {
    pub session_id: SessionId,
    pub commands: Vec<Command>,
    pub game_state: GameState,
    pub events: Vec<GameEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDamageReceivedType {
    pub session_id: SessionId,
    pub damage: Number,
    pub game_state: GameState,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EndType {
    pub winner_id: SessionId,
    pub winner_info: BotInfo,
    pub room_data: RoomData,
}
