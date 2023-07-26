use bevy::{prelude::*, window::PrimaryWindow};
use bevy_prototype_lyon::{prelude::*, shapes::RoundedPolygon};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub(crate) struct DrawPointer {
    pub(crate) points: Vec<Vec2>,
}

impl DrawPointer {
    pub(crate) fn new(initial_point: Vec2) -> Self {
        Self {
            points: vec![initial_point],
        }
    }
}

pub(crate) fn start_draw(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_pos) = cursor_position(window, camera, camera_transform) else { return; };

    commands
        .spawn(SpatialBundle::default())
        .insert(DrawPointer::new(cursor_pos));
}

pub(crate) fn draw(
    mut commands: Commands,
    mut draw_pointer_query: Query<(&mut DrawPointer, Entity)>,
    mouse_input: Res<Input<MouseButton>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    if !mouse_input.pressed(MouseButton::Left)
        || mouse_input.just_pressed(MouseButton::Left)
        || mouse_input.just_released(MouseButton::Left)
        || draw_pointer_query.is_empty()
    {
        return;
    }

    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    let Some(cursor_pos) = cursor_position(window, camera, camera_transform) else { return; };

    let (mut draw_pointer, draw_pointer_entity) = draw_pointer_query.single_mut();
    let from = *draw_pointer.points.last().unwrap();
    let to = cursor_pos;

    if from.distance(to) < 10. {
        return;
    }

    let mut path_builder = PathBuilder::new();

    path_builder.move_to(from);
    path_builder.line_to(to);
    let path = path_builder.build();

    commands
        .entity(draw_pointer_entity)
        .with_children(|parent| {
            parent.spawn((
                ShapeBundle {
                    path,
                    ..Default::default()
                },
                Stroke::new(Color::WHITE, 10.),
                Fill::color(Color::NONE),
            ));
        });

    draw_pointer.points.push(to);
}

pub(crate) fn finish_draw(
    mut commands: Commands,
    draw_pointer_query: Query<(Entity, &DrawPointer)>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if !mouse_input.just_released(MouseButton::Left) {
        return;
    }

    let (draw_pointer_entity, draw_pointer) = draw_pointer_query.single();
    let mut draw_pointer_commands = commands.entity(draw_pointer_entity);
    draw_pointer_commands.remove::<DrawPointer>();

    let points = &draw_pointer.points;
    if points.len() < 2 {
        return;
    }

    let colliders = (0..(draw_pointer.points.len() - 1))
        .map(|point_i| {
            (
                Vec2::ZERO,
                0.,
                Collider::capsule(points[point_i], points[point_i + 1], 5.),
            )
        })
        .collect();

    draw_pointer_commands
        .insert(RigidBody::Dynamic)
        .insert(Collider::compound(colliders))
        .insert(GravityScale(1.))
        .insert(ColliderMassProperties::Density(10.));
}

fn cursor_position(
    window: &Window,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    let Some(cursor_window_pos) = window.cursor_position() else { return None; };
    camera.viewport_to_world_2d(camera_transform, cursor_window_pos)
}
