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
    initial_pps: Number,
    final_pps: Number,
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

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]

type Block = Option<Piece>;
// pub enum Block {
//     Piece, G, Null
// }

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PieceData {
    piece: Piece,
    x: Number,
    y: Number,
    rotation: i8
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GarbageLine {
    delay: Number
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    board: Vec<Vec<Block>>,
    queue: Vec<Piece>,
    garbage_queued: Vec<GarbageLine>,
    held: Option<Piece>,
    current: PieceData,
    can_hold: bool,
    combo: Number,
    b2b: bool,
    score: Number,
    pieces_placed: Number,
    dead: bool,
}

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
