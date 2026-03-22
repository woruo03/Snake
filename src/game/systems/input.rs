use bevy::{app::AppExit, prelude::*};

use crate::game::{
    components::SnakeHead, messages::ResetRequestedMsg, resources::PerfStats, state::GameState,
};

pub(crate) fn global_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit_writer: MessageWriter<AppExit>,
    mut perf: ResMut<PerfStats>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit_writer.write(AppExit::Success);
    }

    if keyboard.just_pressed(KeyCode::F3) {
        perf.show_overlay = !perf.show_overlay;
    }
}

pub(crate) fn menu_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

pub(crate) fn restart_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut reset_writer: MessageWriter<ResetRequestedMsg>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }

    match game_state.get() {
        GameState::Playing => {
            reset_writer.write(ResetRequestedMsg);
        }
        GameState::GameOver => {
            next_state.set(GameState::Playing);
        }
        GameState::Menu => {}
    }
}

pub(crate) fn snake_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut SnakeHead>,
) {
    let Ok(mut head) = query.single_mut() else {
        return;
    };

    let mut wanted = head.next_direction;

    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
        wanted = crate::game::state::Direction::Up;
    } else if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        wanted = crate::game::state::Direction::Down;
    } else if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) {
        wanted = crate::game::state::Direction::Left;
    } else if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD) {
        wanted = crate::game::state::Direction::Right;
    }

    if !wanted.is_opposite(head.direction) {
        head.next_direction = wanted;
    }
}
