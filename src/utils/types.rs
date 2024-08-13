use serde::{Deserialize, Serialize};
use serde_json::Number;

pub type SessionId = String;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomData {
    id: String,
    host: PlayerInfo,
    private: bool,
    ft: Number,
    pps: Number,
    initial_multiplier: Number,
    final_multiplier: Number,
    start_margin: Number,
    end_margin: Number,
    max_players: Number,
    game_ongoing: bool,
    round_ongoing: bool,
    started_at: Option<Number>,
    ended_at: Option<Number>,
    last_winner: Option<SessionId>,
    players: Vec<PlayerData>,
    banned: Vec<PlayerInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerData {
    session_id: SessionId,
    playing: bool,
    info: PlayerInfo,
    wins: Number,
    game_state: Option<GameState>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo {
    user_id: String,
    creator: String,
    bot: String,
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
    delay: Number
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
    clear_name: ClearName,
    all_spin: bool,
    b2b: bool,
    combo: Number,
    pc: bool,
    attack: Number,
    cancelled: Number,
    piece: PieceData,
    cleared_lines: Vec<ClearedLines>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClearedLines {
    height: Number,
    blocks: Vec<Block>
}
