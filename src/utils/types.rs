use serde::{Deserialize, Serialize};
use serde_json::Number;

use super::piece::*;

pub type SessionId = String;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HostInfo {
    id: String,
    display_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomSettings {
    pub private: bool,
    pub ft: Number,
    pub pps: Number,
    pub initial_multiplier: Number,
    pub final_multiplier: Number,
    pub start_margin: Number,
    pub end_margin: Number,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomData {
    pub id: String,
    pub host: HostInfo,
    pub settings: RoomSettings,
    pub game_ongoing: bool,
    pub round_ongoing: bool,
    pub started_at: Option<Number>,
    pub ended_at: Option<Number>,
    pub last_winner: Option<SessionId>,
    pub players: Vec<PlayerData>,
    pub banned: Vec<BotInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerData {
    pub session_id: SessionId,
    pub playing: bool,
    pub info: BotInfo,
    pub wins: Number,
    pub game_state: Option<GameState>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BotInfo {
    pub id: String,
    pub name: String,
    pub avatar: Vec<Vec<Option<Piece>>>,
    pub team: Option<String>,
    pub language: Option<String>,
    pub eval: Option<String>,
    pub movegen: Option<String>,
    pub search: Option<String>,
    pub developers: Vec<DeveloperInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperInfo {
    pub id: String,
    pub display_name: String,
}

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub enum Piece {
//     I, O, J, L, S, Z, T,
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PieceData {
    pub piece: Piece,
    pub x: i16,
    pub y: i16,
    pub rotation: u16
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GarbageLine {
    pub delay: Number
}

pub type Board = Vec<[Option<Block>; 10]>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub board: Board,
    pub queue: Vec<Piece>,
    pub garbage_queued: Vec<GarbageLine>,
    pub held: Option<Piece>,
    pub current: PieceData,
    pub can_hold: bool,
    pub combo: Number,
    pub b2b: bool,
    pub score: Number,
    pub pieces_placed: Number,
    pub dead: bool,
}

/// Represents the type of command to be sent to the 
/// server and what move you want the piece to make
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Hold,
    MoveLeft,
    MoveRight,
    SonicLeft,
    SonicRight,
    RotateCw,
    RotateCcw,
    Drop,
    SonicDrop,
    HardDrop,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ClearName {
    #[serde(rename = "Single")]
    Single,
    #[serde(rename = "Double")]
    Double,
    #[serde(rename = "Triple")]
    Triple,
    #[serde(rename = "Quad")]
    Quad,
    #[serde(rename = "All-Spin Single")]
    ASS,
    #[serde(rename = "All-Spin Double")]
    ASD,
    #[serde(rename = "All-Spin Triple")]
    AST,
    #[serde(rename = "Perfect Clear")]
    PC,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GameEvent {
    PiecePlaced {payload: PiecePlacedType},
    QueueAdded {payload: QueueAddedType},
    DamageTanked {payload: DamageTankedType},
    Clear {payload: ClearType},
    GameOver,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PiecePlacedType {
    initial: PieceData,
    r#final: PieceData,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueAddedType {
    piece: Piece
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageTankedType {
    hole_indices: Vec<Number>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearType {
    pub clear_name: ClearName,
    pub all_spin: bool,
    pub b2b: bool,
    pub combo: Number,
    pub pc: bool,
    pub attack: Number,
    pub cancelled: Number,
    pub piece: PieceData,
    pub cleared_lines: Vec<ClearedLines>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearedLines {
    pub height: Number,
    pub blocks: Vec<Option<Block>>,
}
