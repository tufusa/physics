use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config;

pub(crate) fn spawn(mut commands: Commands) {
    let wall_x =
        config::Window::SIZE.x / 2. + config::Wall::THICKNESS / 2. - config::Wall::VISIBLE_WIDTH;
    let wall_y =
        config::Window::SIZE.y / 2. + config::Wall::THICKNESS / 2. - config::Wall::VISIBLE_WIDTH;

    commands
        .spawn(Collider::cuboid(
            config::Window::SIZE.x,
            config::Wall::THICKNESS / 2.,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(0., -wall_y, 0.)))
        .insert(Restitution::coefficient(1.));
    commands
        .spawn(Collider::cuboid(
            config::Window::SIZE.x,
            config::Wall::THICKNESS / 2.,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(0., wall_y, 0.)))
        .insert(Restitution::coefficient(1.));
    commands
        .spawn(Collider::cuboid(
            config::Wall::THICKNESS / 2.,
            config::Window::SIZE.y,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(-wall_x, 0., 0.)))
        .insert(Restitution::coefficient(1.));
    commands
        .spawn(Collider::cuboid(
            config::Wall::THICKNESS / 2.,
            config::Window::SIZE.y,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(wall_x, 0., 0.)))
        .insert(Restitution::coefficient(1.));
}
