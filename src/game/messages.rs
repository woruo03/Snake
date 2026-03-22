use bevy::prelude::*;

use crate::game::components::GridPos;

#[derive(Message, Debug, Clone, Copy)]
pub(crate) struct GameOverMsg;

#[derive(Message, Debug, Clone, Copy)]
pub(crate) struct FoodEatenMsg {
    pub(crate) position: GridPos,
}

#[derive(Message, Debug, Clone, Copy)]
pub(crate) struct ScoreChangedMsg {
    pub(crate) score: u32,
}

#[derive(Message, Debug, Clone, Copy)]
pub(crate) struct ResetRequestedMsg;

#[derive(Message, Debug, Clone, Copy)]
pub(crate) struct SnakeMovedMsg {
    pub(crate) from: GridPos,
    pub(crate) to: GridPos,
}
