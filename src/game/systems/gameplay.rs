use std::collections::HashSet;

use bevy::prelude::*;

use crate::game::{
    components::{Food, GridPos, SnakeHead, SnakeSegment},
    constants::{CELL_FILL, SNAKE_SEGMENT_COLOR, Z_SNAKE_BODY, Z_SNAKE_HEAD},
    messages::{FoodEatenMsg, GameOverMsg, ScoreChangedMsg, SnakeMovedMsg},
    resources::{BoardBounds, GameConfig, GameStatus, PendingGrowth, RngState, SnakeBody},
    state::GameState,
};

use super::common::{grid_to_world, spawn_food_random};

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn snake_movement_system(
    mut commands: Commands,
    mut head_query: Query<
        (&mut SnakeHead, &mut GridPos, &mut Transform),
        (With<SnakeHead>, Without<SnakeSegment>),
    >,
    mut segment_queries: ParamSet<(
        Query<&GridPos, (With<SnakeSegment>, Without<SnakeHead>)>,
        Query<(&mut GridPos, &mut Transform), (With<SnakeSegment>, Without<SnakeHead>)>,
    )>,
    mut snake_body: ResMut<SnakeBody>,
    mut pending_growth: ResMut<PendingGrowth>,
    config: Res<GameConfig>,
    mut moved_writer: MessageWriter<SnakeMovedMsg>,
    mut previous_chain: Local<Vec<GridPos>>,
) {
    let Ok((mut head, mut head_pos, mut head_transform)) = head_query.single_mut() else {
        return;
    };

    head.direction = head.next_direction;

    let previous_head = *head_pos;
    let delta = head.direction.delta();
    head_pos.x += delta.x;
    head_pos.y += delta.y;

    head_transform.translation = grid_to_world(*head_pos, config.grid_size).extend(Z_SNAKE_HEAD);

    previous_chain.clear();
    previous_chain.reserve(snake_body.0.len() + 1);
    previous_chain.push(previous_head);

    {
        let segment_pos_query = segment_queries.p0();
        for entity in &snake_body.0 {
            if let Ok(pos) = segment_pos_query.get(*entity) {
                previous_chain.push(*pos);
            }
        }
    }

    {
        let mut segment_update_query = segment_queries.p1();
        for (index, entity) in snake_body.0.iter().enumerate() {
            if let Ok((mut pos, mut transform)) = segment_update_query.get_mut(*entity) {
                let new_pos = previous_chain[index];
                *pos = new_pos;
                transform.translation =
                    grid_to_world(new_pos, config.grid_size).extend(Z_SNAKE_BODY);
            }
        }
    }

    if pending_growth.0 > 0 {
        let tail_spawn_pos = previous_chain.last().copied().unwrap_or(previous_head);
        let segment = commands
            .spawn((
                Sprite::from_color(
                    SNAKE_SEGMENT_COLOR,
                    Vec2::splat(config.grid_size * CELL_FILL),
                ),
                Transform::from_translation(
                    grid_to_world(tail_spawn_pos, config.grid_size).extend(Z_SNAKE_BODY),
                ),
                SnakeSegment,
                tail_spawn_pos,
                crate::game::components::GameEntity,
            ))
            .id();

        snake_body.0.push(segment);
        pending_growth.0 -= 1;
    }

    moved_writer.write(SnakeMovedMsg {
        from: previous_head,
        to: *head_pos,
    });
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn food_collision_system(
    mut commands: Commands,
    head_query: Query<&GridPos, With<SnakeHead>>,
    food_query: Query<(Entity, &GridPos), With<Food>>,
    segment_query: Query<&GridPos, With<SnakeSegment>>,
    mut pending_growth: ResMut<PendingGrowth>,
    mut status: ResMut<GameStatus>,
    bounds: Res<BoardBounds>,
    config: Res<GameConfig>,
    mut rng: ResMut<RngState>,
    mut food_eaten_writer: MessageWriter<FoodEatenMsg>,
    mut score_writer: MessageWriter<ScoreChangedMsg>,
) {
    let Ok(head_pos) = head_query.single() else {
        return;
    };

    for (food_entity, food_pos) in &food_query {
        if *head_pos == *food_pos {
            commands.entity(food_entity).despawn();
            pending_growth.0 += 1;

            status.score = status.score.saturating_add(1);
            status.high_score = status.high_score.max(status.score);

            food_eaten_writer.write(FoodEatenMsg {
                position: *food_pos,
            });
            score_writer.write(ScoreChangedMsg {
                score: status.score,
            });

            // 只在吃到食物时生成新食物，避免每个 fixed tick 都做空查询。
            let mut occupied = HashSet::new();
            occupied.insert((head_pos.x, head_pos.y));
            for segment_pos in &segment_query {
                occupied.insert((segment_pos.x, segment_pos.y));
            }
            spawn_food_random(&mut commands, &config, &bounds, &mut rng, &occupied);
            break;
        }
    }
}

pub(crate) fn collision_system(
    head_query: Query<&GridPos, With<SnakeHead>>,
    segment_query: Query<&GridPos, With<SnakeSegment>>,
    bounds: Res<BoardBounds>,
    mut game_over_writer: MessageWriter<GameOverMsg>,
) {
    let Ok(head_pos) = head_query.single() else {
        return;
    };

    if head_pos.x < bounds.min_x
        || head_pos.x > bounds.max_x
        || head_pos.y < bounds.min_y
        || head_pos.y > bounds.max_y
    {
        game_over_writer.write(GameOverMsg);
        return;
    }

    for segment_pos in &segment_query {
        if *segment_pos == *head_pos {
            game_over_writer.write(GameOverMsg);
            return;
        }
    }
}

pub(crate) fn game_over_system(
    mut reader: MessageReader<GameOverMsg>,
    state: Res<State<GameState>>,
    mut status: ResMut<GameStatus>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut hit = false;
    for _ in reader.read() {
        hit = true;
    }

    if hit && *state.get() == GameState::Playing {
        status.is_game_over = true;
        next_state.set(GameState::GameOver);
    }
}
