use crate::{
    plugins,
    resources::BrickMaterials,
    systems::{
        check_for_collisions, esc_exit, move_ball, move_player, populate_brick_materials,
        spawn_ball, spawn_bricks, spawn_camera, spawn_player, spawn_walls,
    },
};

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPlugins.set(plugins::window_plugin()),))
            .add_plugins(EguiPlugin::default())
            .add_plugins(
                WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Enter)),
            )
            .insert_resource(BrickMaterials::new())
            .add_systems(Startup, populate_brick_materials)
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_bricks.after(populate_brick_materials))
            .add_systems(Startup, spawn_walls)
            .add_systems(Startup, spawn_player)
            .add_systems(Startup, spawn_ball)
            .add_systems(Update, esc_exit)
            .add_systems(FixedUpdate, check_for_collisions)
            .add_systems(FixedUpdate, move_player)
            .add_systems(FixedUpdate, move_ball);
    }
}
