use bevy::{
    prelude::*,
    time::FixedTimestep,
};

const TIME_STEP: f32 = 1.0 / 60.;
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.3, 0.3);
const TETROMINO_COLOR: Color = Color::rgb(0.2, 0.2, 0.9);
const TETROMINO_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0., -1.);
const LEFT_BOUND: f32 = -400.;
const RIGHT_BOUND: f32 = 400.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(window_resize_system)
        .insert_resource(Tick(0))
        .insert_resource(KeyDebounce(false))
        .insert_resource(GravityDebounce(false))
        .insert_resource(PendingInertia(0.))
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(ticker)
                .with_system(apply_velocity)
                .with_system(move_tetromino)
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}

#[derive(Resource)]
struct Tick(u64);

#[derive(Resource)]
struct PendingInertia(f32);

#[derive(Resource)]
struct KeyDebounce(bool);

#[derive(Resource)]
struct GravityDebounce(bool);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Tetromino;

#[derive(Component)]
struct Wall;

// Add the game's entities to our world
fn setup(
    mut commands: Commands,
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
        Wall,
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

fn ticker(
    mut query: Query<&mut Transform, With<Tetromino>>,
    mut tick: ResMut<Tick>,
    mut key_debounce: ResMut<KeyDebounce>,
    mut gravity_debounce: ResMut<GravityDebounce>,
    mut pending_inertia: ResMut<PendingInertia>,
) {
    tick.0 = (tick.0 + 1) % 1_000_000;

    if tick.0 % 30 == 0 {
        key_debounce.0 = false;

        if pending_inertia.0 != 0. {
            let mut tetromino_transform = query.single_mut();
            let new_tetromino_position = tetromino_transform.translation.x + pending_inertia.0;
            tetromino_transform.translation.x = new_tetromino_position.clamp(LEFT_BOUND, RIGHT_BOUND);

            pending_inertia.0 = 0.;
        }
    }
    if tick.0 % 60 == 0 {
        gravity_debounce.0 = false;
    } else {
        gravity_debounce.0 = true;
    }
}

fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    gravity_debounce: Res<GravityDebounce>,
) {
    if gravity_debounce.0 { return; }
    
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn move_tetromino(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Tetromino>>,
    mut key_debounce: ResMut<KeyDebounce>,
    mut pending_inertia: ResMut<PendingInertia>,
) {
    let mut tetromino_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 20.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 20.0;
    }

    if direction != 0. && key_debounce.0 {
        pending_inertia.0 = direction;
        return;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_tetromino_position = tetromino_transform.translation.x + direction;

    tetromino_transform.translation.x = new_tetromino_position.clamp(LEFT_BOUND, RIGHT_BOUND);

    key_debounce.0 = true;
}
