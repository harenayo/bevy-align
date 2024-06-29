use {
    bevy::{
        app::{
            App,
            Startup,
            Update,
        },
        core_pipeline::core_2d::Camera2dBundle,
        ecs::{
            component::Component,
            query::With,
            system::{
                Commands,
                Query,
                Res,
            },
        },
        math::{
            bool::BVec3,
            f32::Vec3,
        },
        render::color::Color,
        sprite::{
            Sprite,
            SpriteBundle,
        },
        time::Time,
        transform::components::Transform,
        DefaultPlugins,
    },
    bevy_align::{
        AlignPlugin,
        Aligning,
    },
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, AlignPlugin))
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component)]
struct Rotating;

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let rotating = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    ..Sprite::default()
                },
                transform: Transform::from_scale(Vec3::splat(20.0)),
                ..SpriteBundle::default()
            },
            Rotating,
        ))
        .id();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Sprite::default()
            },
            transform: Transform {
                translation: Vec3::new(150.0, 0.0, 0.0),
                scale: Vec3::splat(20.0),
                ..Transform::default()
            },
            ..SpriteBundle::default()
        },
        Aligning {
            target: rotating,
            enabled: BVec3::new(false, true, false),
        },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Sprite::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 150.0, 0.0),
                scale: Vec3::splat(20.0),
                ..Transform::default()
            },
            ..SpriteBundle::default()
        },
        Aligning {
            target: rotating,
            enabled: BVec3::new(true, false, false),
        },
    ));
}

fn update(mut transforms: Query<&mut Transform, With<Rotating>>, time: Res<Time>) {
    for mut transform in &mut transforms {
        transform.translation.x = 100.0 * time.elapsed_seconds().cos();
        transform.translation.y = 100.0 * time.elapsed_seconds().sin();
    }
}
