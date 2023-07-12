use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod config;
mod in_game;
mod plugins;

fn main() {
    App::new()
        .add_plugin(plugins::Base)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(in_game::walls::spawn)
        .add_startup_system(spawn)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    in_game::pyramid::spawn(&mut commands, &mut meshes, &mut materials);
    in_game::ball::spawn(&mut commands, &mut meshes, &mut materials);
}
