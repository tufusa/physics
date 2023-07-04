use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::config;

pub(crate) fn spawn(commands: &mut Commands) {
    let wall_x = config::Window::SIZE.x / 2. + config::Wall::THICKNESS;
    let wall_y = config::Window::SIZE.y / 2. + config::Wall::THICKNESS;

    commands
        .spawn(Collider::cuboid(
            config::Window::SIZE.x,
            config::Wall::THICKNESS,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.,
            -wall_y + config::Wall::VISIBLE_WIDTH,
            0.,
        )))
        .insert(Restitution::coefficient(1.));
    commands
        .spawn(Collider::cuboid(
            config::Window::SIZE.x,
            config::Wall::THICKNESS,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.,
            wall_y - config::Wall::VISIBLE_WIDTH,
            0.,
        )))
        .insert(Restitution::coefficient(1.));
    commands
        .spawn(Collider::cuboid(
            config::Wall::THICKNESS,
            config::Window::SIZE.y,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            -wall_x + config::Wall::VISIBLE_WIDTH,
            0.,
            0.,
        )))
        .insert(Restitution::coefficient(1.));
    commands
        .spawn(Collider::cuboid(
            config::Wall::THICKNESS,
            config::Window::SIZE.y,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            wall_x - config::Wall::VISIBLE_WIDTH,
            0.,
            0.,
        )))
        .insert(Restitution::coefficient(1.));
}
