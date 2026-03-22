mod game;

use bevy::prelude::*;
use game::SnakeGamePlugin;

fn main() {
    App::new().add_plugins(SnakeGamePlugin).run();
}
