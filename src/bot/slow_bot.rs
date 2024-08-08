use crate::utils::event_types::{ActionType, RequestMoveType};
use crate::utils::types::Command;
use std::thread::sleep;
use std::time::Duration;

pub fn request_moves(_event: &RequestMoveType) -> ActionType {
    let mut actions = ActionType::new();
    actions.push(Command::MoveLeft);
    actions.push(Command::MoveRight);

    let time = Duration::new(3, 500000000);
    sleep(time);

    actions
}
