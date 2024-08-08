use crate::utils::event_types::{ActionType, RequestMoveType};
use std::thread::sleep;
use std::time::Duration;

pub fn request_moves(event: &RequestMoveType) -> ActionType {
    let actions = ActionType::new();

    let time = Duration::new(3, 500000000);
    sleep(time);

    actions
}
