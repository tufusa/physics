use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::config;

pub(crate) fn spawn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    translation: Vec3,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(config::Square::SIDE, config::Square::SIDE, 0.).into())
                .into(),
            material: materials.add(ColorMaterial::from(config::Square::COLOR)),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            config::Square::SIDE / 2.,
            config::Square::SIDE / 2.,
        ))
        // .insert(Restitution::coefficient(0.95)) // 反発係数
        .insert(TransformBundle::from(Transform::from_translation(
            translation,
        )))
        // .insert(GravityScale(1.))
        // .insert(Friction::coefficient(0.01))
        .insert(ColliderMassProperties::Density(1.));
}
