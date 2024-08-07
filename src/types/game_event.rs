use super::types::*;

use serde::{Deserialize, Serialize};
use serde_json::Number;

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

