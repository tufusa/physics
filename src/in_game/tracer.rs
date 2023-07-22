use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub(crate) struct Tracer {
    pub(crate) last_point: Vec2,
}

impl Tracer {
    pub(crate) fn new(initial_translation: Vec2) -> Self {
        Self {
            last_point: initial_translation,
        }
    }
}

pub(crate) fn trace(mut commands: Commands, mut tracer_query: Query<(&Transform, &mut Tracer)>) {
    tracer_query.iter_mut().for_each(|(transform, mut tracer)| {
        let from = tracer.last_point;
        let to = transform.translation.truncate();

        if from.distance(to) < 10. {
            return;
        }

        let mut path_builder = PathBuilder::new();

        path_builder.move_to(from);
        path_builder.line_to(to);
        let path = path_builder.build();

        commands
            .spawn((
                ShapeBundle {
                    path,
                    ..Default::default()
                },
                Stroke::new(Color::WHITE, 1.),
                Fill::color(Color::NONE),
            ))
            .insert(SpatialBundle::default())
            .insert(RigidBody::Dynamic)
            .insert(Collider::capsule(from, to, 0.5));

        tracer.last_point = to;
    });
}
