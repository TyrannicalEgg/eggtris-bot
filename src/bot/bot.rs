use crate::utils::event_types::{ActionType, RequestMoveType};

pub trait BotTrait {
    fn request_moves(event: &RequestMoveType) -> ActionType;
}

pub struct Bot {}


