pub(crate) mod components;
pub(crate) mod constants;
pub(crate) mod messages;
pub(crate) mod resources;
pub(crate) mod state;
pub(crate) mod systems;

use bevy::{prelude::*, time::Fixed};

use constants::{SNAKE_FIXED_HZ, WINDOW_HEIGHT, WINDOW_WIDTH};
use messages::{FoodEatenMsg, GameOverMsg, ResetRequestedMsg, ScoreChangedMsg, SnakeMovedMsg};
use resources::{
    BoardBounds, CameraShake, GameConfig, GameStatus, PendingGrowth, PerfStats, RngState, SnakeBody,
};
use state::GameState;
use systems::{effects, gameplay, input, perf, reset, setup, ui};

pub(crate) struct SnakeGamePlugin;

impl Plugin for SnakeGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(constants::BG_COLOR))
            .insert_resource(GameConfig::default())
            .insert_resource(GameStatus::default())
            .insert_resource(BoardBounds::from_window())
            .insert_resource(SnakeBody::default())
            .insert_resource(PendingGrowth::default())
            .insert_resource(RngState::seeded(0xC0DEC0FFEE))
            .insert_resource(PerfStats::default())
            .insert_resource(CameraShake::default())
            .insert_resource(Time::<Fixed>::from_hz(SNAKE_FIXED_HZ))
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Snake (Bevy 0.18.1)".to_string(),
                    resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }))
            .init_state::<GameState>()
            .add_message::<GameOverMsg>()
            .add_message::<FoodEatenMsg>()
            .add_message::<ScoreChangedMsg>()
            .add_message::<ResetRequestedMsg>()
            .add_message::<SnakeMovedMsg>()
            .add_systems(
                Startup,
                (
                    setup::setup_camera,
                    setup::setup_hud,
                    setup::setup_game_over_text,
                    setup::setup_perf_overlay,
                    setup::spawn_background_grid,
                ),
            )
            .add_systems(OnEnter(GameState::Menu), setup::spawn_menu_ui)
            .add_systems(OnExit(GameState::Menu), setup::cleanup_menu_ui)
            .add_systems(OnEnter(GameState::Playing), reset::reset_round_system)
            .add_systems(OnEnter(GameState::GameOver), setup::show_game_over_text)
            .add_systems(OnExit(GameState::GameOver), setup::hide_game_over_text)
            .add_systems(
                Update,
                (
                    input::global_input_system,
                    input::menu_input_system.run_if(in_state(GameState::Menu)),
                    input::restart_input_system,
                    input::snake_input_system.run_if(in_state(GameState::Playing)),
                    reset::handle_runtime_restart_system.run_if(in_state(GameState::Playing)),
                    gameplay::game_over_system,
                    ui::apply_score_changed_ui_system,
                    effects::spawn_food_eaten_effect_system,
                    effects::spawn_trail_effect_system,
                    effects::spawn_collision_feedback_system,
                    effects::update_pulse_effect_system,
                    effects::update_fade_effect_system,
                    effects::update_ui_float_effect_system,
                    effects::lifetime_cleanup_system,
                    effects::update_camera_shake_system,
                    perf::sample_perf_stats_system,
                    perf::update_perf_overlay_system,
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    gameplay::snake_movement_system,
                    gameplay::food_collision_system,
                    gameplay::collision_system,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
