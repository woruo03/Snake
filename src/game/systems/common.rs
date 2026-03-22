use std::collections::HashSet;

use bevy::prelude::*;

use crate::game::{
    components::{Food, GameEntity, GridPos, SnakeHead, SnakeSegment},
    constants::{
        CELL_FILL, FOOD_COLOR, SNAKE_HEAD_COLOR, SNAKE_SEGMENT_COLOR, Z_FOOD, Z_SNAKE_BODY,
        Z_SNAKE_HEAD,
    },
    resources::{BoardBounds, GameConfig, RngState, SnakeBody},
    state::Direction,
};

pub(crate) fn grid_to_world(pos: GridPos, grid_size: f32) -> Vec2 {
    Vec2::new(pos.x as f32 * grid_size, pos.y as f32 * grid_size)
}

pub(crate) fn clear_game_entities(
    commands: &mut Commands,
    query: &Query<Entity, With<GameEntity>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub(crate) fn spawn_initial_snake(
    commands: &mut Commands,
    config: &GameConfig,
    snake_body: &mut SnakeBody,
) -> HashSet<(i32, i32)> {
    let mut occupied = HashSet::new();

    let head_pos = GridPos { x: 0, y: 0 };
    commands.spawn((
        Sprite::from_color(SNAKE_HEAD_COLOR, Vec2::splat(config.grid_size * CELL_FILL)),
        Transform::from_translation(grid_to_world(head_pos, config.grid_size).extend(Z_SNAKE_HEAD)),
        SnakeHead {
            direction: Direction::Right,
            next_direction: Direction::Right,
        },
        head_pos,
        GameEntity,
    ));
    occupied.insert((head_pos.x, head_pos.y));

    for i in 1..config.initial_snake_length {
        let segment_pos = GridPos {
            x: -(i as i32),
            y: 0,
        };

        let segment = commands
            .spawn((
                Sprite::from_color(
                    SNAKE_SEGMENT_COLOR,
                    Vec2::splat(config.grid_size * CELL_FILL),
                ),
                Transform::from_translation(
                    grid_to_world(segment_pos, config.grid_size).extend(Z_SNAKE_BODY),
                ),
                SnakeSegment,
                segment_pos,
                GameEntity,
            ))
            .id();

        snake_body.0.push(segment);
        occupied.insert((segment_pos.x, segment_pos.y));
    }

    occupied
}

pub(crate) fn spawn_food_random(
    commands: &mut Commands,
    config: &GameConfig,
    bounds: &BoardBounds,
    rng: &mut RngState,
    occupied: &HashSet<(i32, i32)>,
) {
    let total_cells = (bounds.width() * bounds.height()).max(0) as usize;
    if occupied.len() >= total_cells {
        return;
    }

    let mut attempts = 0;
    let max_attempts = 256;

    while attempts < max_attempts {
        let x = rng.range_i32(bounds.min_x, bounds.max_x);
        let y = rng.range_i32(bounds.min_y, bounds.max_y);

        if !occupied.contains(&(x, y)) {
            let pos = GridPos { x, y };
            commands.spawn((
                Sprite::from_color(FOOD_COLOR, Vec2::splat(config.grid_size * CELL_FILL)),
                Transform::from_translation(grid_to_world(pos, config.grid_size).extend(Z_FOOD)),
                Food,
                pos,
                GameEntity,
            ));
            return;
        }

        attempts += 1;
    }

    // 兜底：顺序扫描，确保一定能在可用格子里生成。
    for y in bounds.min_y..=bounds.max_y {
        for x in bounds.min_x..=bounds.max_x {
            if occupied.contains(&(x, y)) {
                continue;
            }

            let pos = GridPos { x, y };
            commands.spawn((
                Sprite::from_color(FOOD_COLOR, Vec2::splat(config.grid_size * CELL_FILL)),
                Transform::from_translation(grid_to_world(pos, config.grid_size).extend(Z_FOOD)),
                Food,
                pos,
                GameEntity,
            ));
            return;
        }
    }
}
