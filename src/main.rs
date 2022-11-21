use bevy::{
    prelude::*,
    time::FixedTimestep,
};

const TETROMINOS: [i32; 3] = [1, 2, 3];
const TIME_STEP: f32 = 1.0;
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.3, 0.3);
const TETROMINO_COLOR: Color = Color::rgb(0.2, 0.2, 0.9);
const TETROMINO_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0., -1.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(window_resize_system)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(apply_velocity)
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Tetromino;

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: Vec3::new(500., 35., 0.),
                ..default()
            },
            sprite: Sprite {
                color: TETROMINO_COLOR,
                ..default()
            },
            ..default()
        },
        Tetromino,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 100., 0.0),
                scale: TETROMINO_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: TETROMINO_COLOR,
                ..default()
            },
            ..default()
        },
        Tetromino,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * 20.),
    ));

}

fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    println!("Window size was: {},{}", window.width(), window.height());
    window.set_resolution(500., 800.);
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
