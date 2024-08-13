use serde::{Deserialize, Serialize};
use serde_json::Number;

pub type SessionId = String;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomData {
    pub id: String,
    pub host: PlayerInfo,
    pub private: bool,
    pub ft: Number,
    pub pps: Number,
    pub initial_multiplier: Number,
    pub final_multiplier: Number,
    pub start_margin: Number,
    pub end_margin: Number,
    pub max_players: Number,
    pub game_ongoing: bool,
    pub round_ongoing: bool,
    pub started_at: Option<Number>,
    pub ended_at: Option<Number>,
    pub last_winner: Option<SessionId>,
    pub players: Vec<PlayerData>,
    pub banned: Vec<PlayerInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerData {
    pub session_id: SessionId,
    pub playing: bool,
    pub info: PlayerInfo,
    pub wins: Number,
    pub game_state: Option<GameState>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo {
    pub user_id: String,
    pub creator: String,
    pub bot: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Piece {
    I, O, J, L, S, Z, T
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Block {
    I, O, J, L, S, Z, T, G
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PieceData {
    pub piece: Piece,
    pub x: Number,
    pub y: Number,
    pub rotation: i8
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GarbageLine {
    pub delay: Number
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub board: Vec<[Option<Block>; 10]>,
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
    DamageTanked {payload: DamageTankedType},
    Clear,
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
    pub cleared_lines: Vec<ClearedLines>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearedLines {
    pub height: Number,
    pub blocks: Vec<Block>
}
