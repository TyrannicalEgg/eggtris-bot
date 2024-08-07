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
#[serde(rename_all = "camelCase")]
pub enum Piece {
    I, O, J, L, S, Z, T
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Block {
    Piece, G, Null
}

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
    b2b: Number,
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
}


pub enum ClearName {
    Single,
    Double,
    Triple,
    Quad,
    ASS,
    ASD,
    AST,
    PC,
}

impl std::fmt::Display for ClearName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single => write!(f, "Single"),
            Self::Double => write!(f, "Double"),
            Self::Triple => write!(f, "Triple"),
            Self::Quad => write!(f, "Quad"),
            Self::ASS => write!(f, "All-Spin Single"),
            Self::ASD => write!(f, "All-Spin Double"),
            Self::AST => write!(f, "All-Spin Triple"),
            Self::PC => write!(f, "Perfect Clear"),
        }
    }
}



#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameEvent {

}
