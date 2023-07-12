use bevy::prelude::*;

use crate::{config, in_game};

pub(crate) fn spawn(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    for level in 0..config::Pyramid::STEP {
        for offset in 0..=level {
            in_game::square::spawn(commands, meshes, materials, position(level, offset))
        }
    }
}

fn position(current_level: u32, offset: u32) -> Vec3 {
    Vec3 {
        x: (offset as f32 - current_level as f32 / 2.) * config::Square::SIDE,
        y: config::Pyramid::BASE_Y
            + (config::Pyramid::STEP - current_level - 1) as f32 * config::Square::SIDE
            + config::Square::SIDE / 2.,
        z: 0.,
    }
}
