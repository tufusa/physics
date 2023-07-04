use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
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
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    in_game::walls::spawn(&mut commands);
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Circle {
                        radius: 50.,
                        ..Default::default()
                    }
                    .into(),
                )
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(50.))
        .insert(Restitution::coefficient(1.)) // 反発係数
        .insert(TransformBundle::from(Transform::from_xyz(0., 300., 0.)))
        .insert(GravityScale(0.))
        .insert(Velocity {
            linvel: (0., -150.).into(),
            angvel: 50.,
        })
        // .insert(Friction {
        // coefficient: 0.,
        // combine_rule: CoefficientCombineRule::Min,
        // })
        .insert(ColliderMassProperties::Density(1.));
}
