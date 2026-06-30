mod components;
mod enums;
mod events;
mod plugins;
mod resources;
mod scenes;
mod systems;

use bevy::prelude::*;

fn main() -> AppExit {
    App::new().add_plugins(plugins::SetupPlugin).run()
}
