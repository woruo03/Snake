use bevy::{ecs::system::SystemParam, prelude::*};

use crate::game::{
    components::GameEntity,
    messages::{ResetRequestedMsg, ScoreChangedMsg},
    resources::{BoardBounds, GameConfig, GameStatus, PendingGrowth, RngState, SnakeBody},
};

use super::common::{clear_game_entities, spawn_food_random, spawn_initial_snake};

#[derive(SystemParam)]
pub(crate) struct RoundResetParams<'w, 's> {
    commands: Commands<'w, 's>,
    game_entities: Query<'w, 's, Entity, With<GameEntity>>,
    snake_body: ResMut<'w, SnakeBody>,
    pending_growth: ResMut<'w, PendingGrowth>,
    status: ResMut<'w, GameStatus>,
    config: Res<'w, GameConfig>,
    bounds: Res<'w, BoardBounds>,
    rng: ResMut<'w, RngState>,
    score_writer: MessageWriter<'w, ScoreChangedMsg>,
}

pub(crate) fn reset_round_system(mut params: RoundResetParams) {
    perform_round_reset(&mut params);
}

pub(crate) fn handle_runtime_restart_system(
    mut reset_reader: MessageReader<ResetRequestedMsg>,
    mut params: RoundResetParams,
) {
    if reset_reader.is_empty() {
        return;
    }

    // Consume all pending restart requests and restart only once.
    for _ in reset_reader.read() {}

    perform_round_reset(&mut params);
}

fn perform_round_reset(params: &mut RoundResetParams) {
    clear_game_entities(&mut params.commands, &params.game_entities);

    params.snake_body.0.clear();
    params.pending_growth.0 = 0;
    params.status.score = 0;
    params.status.is_game_over = false;

    let occupied =
        spawn_initial_snake(&mut params.commands, &params.config, &mut params.snake_body);
    spawn_food_random(
        &mut params.commands,
        &params.config,
        &params.bounds,
        &mut params.rng,
        &occupied,
    );

    params.score_writer.write(ScoreChangedMsg { score: 0 });
}
