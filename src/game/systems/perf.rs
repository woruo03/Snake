use bevy::prelude::*;

use crate::game::{
    components::{EffectEntity, PerfOverlay, SnakeSegment},
    resources::PerfStats,
    state::GameState,
};

pub(crate) fn sample_perf_stats_system(
    mut perf: ResMut<PerfStats>,
    time: Res<Time>,
    all_entities: Query<Entity>,
    snake_segments: Query<Entity, With<SnakeSegment>>,
    effects: Query<Entity, With<EffectEntity>>,
) {
    perf.frame_count = perf.frame_count.saturating_add(1);
    perf.accum_seconds += time.delta_secs();

    if perf.accum_seconds < 0.25 {
        return;
    }

    perf.fps = perf.frame_count as f32 / perf.accum_seconds;
    perf.frame_count = 0;
    perf.accum_seconds = 0.0;

    if !perf.show_overlay {
        return;
    }

    perf.entity_count = all_entities.iter().count();
    perf.snake_count = snake_segments.iter().count() + 1;
    perf.effect_count = effects.iter().count();
}

pub(crate) fn update_perf_overlay_system(
    perf: Res<PerfStats>,
    mut query: Query<(&mut Text, &mut Visibility), With<PerfOverlay>>,
    state: Res<State<GameState>>,
) {
    let Ok((mut text, mut visibility)) = query.single_mut() else {
        return;
    };

    *visibility = if perf.show_overlay {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    if !perf.show_overlay {
        return;
    }

    text.0 = format!(
        "FPS: {:.1}\nEntities: {}\nSnake Segments: {}\nEffects: {}\nState: {:?}",
        perf.fps,
        perf.entity_count,
        perf.snake_count,
        perf.effect_count,
        state.get()
    );
}
