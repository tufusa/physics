use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::config;

use super::tracer::Tracer;

pub(crate) fn spawn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Circle {
                        radius: config::Ball::RADIUS,
                        ..Default::default()
                    }
                    .into(),
                )
                .into(),
            material: materials.add(ColorMaterial::from(config::Ball::COLOR)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(config::Ball::RADIUS))
        .insert(Restitution::coefficient(1.)) // 反発係数
        .insert(TransformBundle::from(Transform::from_xyz(100., 100., 0.)))
        .insert(GravityScale(1.))
        .insert(Velocity {
            linvel: (-100., -150.).into(),
            angvel: 50.,
        })
        .insert(Friction::coefficient(0.))
        .insert(ColliderMassProperties::Density(100.))
        .insert(Tracer::new(Vec2::new(100., 100.)));
}
