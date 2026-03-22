use bevy::prelude::*;

use crate::game::{
    components::{
        EffectEntity, FadeEffect, GameEntity, GridPos, Lifetime, MainCamera, PulseEffect,
        SnakeHead, UiFloatEffect,
    },
    constants::{CELL_FILL, COLLISION_COLOR, EAT_EFFECT_COLOR, TRAIL_COLOR, Z_EFFECT, Z_TRAIL},
    messages::{FoodEatenMsg, GameOverMsg, SnakeMovedMsg},
    resources::{CameraShake, GameConfig, RngState},
};

use super::common::grid_to_world;

pub(crate) fn spawn_food_eaten_effect_system(
    mut commands: Commands,
    mut reader: MessageReader<FoodEatenMsg>,
    config: Res<GameConfig>,
) {
    for msg in reader.read() {
        let world = grid_to_world(msg.position, config.grid_size);

        commands.spawn((
            Sprite::from_color(
                EAT_EFFECT_COLOR,
                Vec2::splat(config.grid_size * CELL_FILL * 1.15),
            ),
            Transform::from_translation(world.extend(Z_EFFECT)),
            PulseEffect {
                t: 0.0,
                base: 1.0,
                amplitude: 0.35,
                speed: 18.0,
            },
            Lifetime {
                timer: Timer::from_seconds(0.25, TimerMode::Once),
            },
            EffectEntity,
            GameEntity,
        ));

        commands.spawn((
            Text::new("+1"),
            TextFont {
                font_size: 24.0,
                ..default()
            },
            TextColor(Color::srgb(0.3, 0.95, 0.45)),
            Node {
                position_type: PositionType::Absolute,
                top: px(46.0),
                left: px(102.0),
                ..default()
            },
            UiFloatEffect {
                top_px: 46.0,
                speed: 62.0,
            },
            Lifetime {
                timer: Timer::from_seconds(0.6, TimerMode::Once),
            },
            EffectEntity,
            GameEntity,
        ));
    }
}

pub(crate) fn spawn_trail_effect_system(
    mut commands: Commands,
    mut reader: MessageReader<SnakeMovedMsg>,
    config: Res<GameConfig>,
) {
    for msg in reader.read() {
        if msg.from == msg.to {
            continue;
        }

        commands.spawn((
            Sprite::from_color(
                TRAIL_COLOR,
                Vec2::splat(config.grid_size * CELL_FILL * 0.45),
            ),
            Transform::from_translation(grid_to_world(msg.from, config.grid_size).extend(Z_TRAIL)),
            FadeEffect {
                alpha: 0.35,
                speed: 1.05,
            },
            Lifetime {
                timer: Timer::from_seconds(0.8, TimerMode::Once),
            },
            EffectEntity,
            GameEntity,
        ));
    }
}

pub(crate) fn spawn_collision_feedback_system(
    mut commands: Commands,
    mut reader: MessageReader<GameOverMsg>,
    head_query: Query<&GridPos, With<SnakeHead>>,
    config: Res<GameConfig>,
    mut shake: ResMut<CameraShake>,
) {
    let mut triggered = false;
    for _ in reader.read() {
        triggered = true;
    }

    if !triggered {
        return;
    }

    if let Ok(head_pos) = head_query.single() {
        commands.spawn((
            Sprite::from_color(
                COLLISION_COLOR,
                Vec2::splat(config.grid_size * CELL_FILL * 1.5),
            ),
            Transform::from_translation(
                grid_to_world(*head_pos, config.grid_size).extend(Z_EFFECT + 0.5),
            ),
            FadeEffect {
                alpha: 0.55,
                speed: 1.3,
            },
            Lifetime {
                timer: Timer::from_seconds(0.45, TimerMode::Once),
            },
            EffectEntity,
            GameEntity,
        ));
    }

    shake.timer = Some(Timer::from_seconds(0.35, TimerMode::Once));
    shake.intensity = 7.0;
}

pub(crate) fn update_pulse_effect_system(
    mut query: Query<(&mut PulseEffect, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut pulse, mut transform) in &mut query {
        pulse.t += time.delta_secs();
        let scale = pulse.base + pulse.amplitude * (pulse.t * pulse.speed).sin().abs();
        transform.scale = Vec3::splat(scale);
    }
}

pub(crate) fn update_fade_effect_system(
    mut query: Query<(&mut FadeEffect, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut fade, mut sprite) in &mut query {
        fade.alpha = (fade.alpha - time.delta_secs() * fade.speed).max(0.0);
        sprite.color.set_alpha(fade.alpha);
    }
}

pub(crate) fn update_ui_float_effect_system(
    mut query: Query<(&mut UiFloatEffect, &mut Node)>,
    time: Res<Time>,
) {
    for (mut float, mut node) in &mut query {
        float.top_px -= float.speed * time.delta_secs();
        node.top = px(float.top_px);
    }
}

pub(crate) fn lifetime_cleanup_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut query {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn update_camera_shake_system(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    mut shake: ResMut<CameraShake>,
    mut rng: ResMut<RngState>,
    time: Res<Time>,
) {
    let Ok(mut transform) = camera_query.single_mut() else {
        return;
    };

    if let Some(timer) = shake.timer.as_mut() {
        timer.tick(time.delta());

        if timer.is_finished() {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            shake.timer = None;
            return;
        }

        let t = 1.0 - timer.fraction();
        let amount = shake.intensity * t;
        transform.translation.x = rng.range_f32(-amount, amount);
        transform.translation.y = rng.range_f32(-amount, amount);
    }
}
