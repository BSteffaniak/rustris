use bevy::{prelude::*, time::FixedTimestep};

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
        .insert_resource(GameState {
            tick: 0,
            pending_inertia: 0.,
            key_debounce: false,
            gravity_debounce: false,
        })
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(ticker)
                .with_system(apply_velocity)
                .with_system(move_tetromino),
        )
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}

#[derive(Resource)]
struct GameState {
    tick: u64,
    pending_inertia: f32,
    key_debounce: bool,
    gravity_debounce: bool,
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Tetromino;

#[derive(Component)]
struct Wall;

// Add the game's entities to our world
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        new_component(
            Vec3::new(0., 0., 0.),
            Vec3::new(500., 35., 0.),
            Color::rgb(0.2, 0.2, 0.9),
        ),
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

fn new_component(translation: Vec3, scale: Vec3, color: Color) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation,
            scale,
            ..default()
        },
        sprite: Sprite { color, ..default() },
        ..default()
    }
}

fn window_resize_system(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    println!("Window size was: {},{}", window.width(), window.height());
    window.set_resolution(500., 800.);
}

fn ticker(mut query: Query<&mut Transform, With<Tetromino>>, mut state: ResMut<GameState>) {
    state.tick = (state.tick + 1) % 1_000_000;

    if state.tick % 30 == 0 {
        state.key_debounce = false;

        if state.pending_inertia != 0. {
            let mut tetromino_transform = query.single_mut();
            let new_tetromino_position = tetromino_transform.translation.x + state.pending_inertia;
            tetromino_transform.translation.x =
                new_tetromino_position.clamp(LEFT_BOUND, RIGHT_BOUND);

            state.pending_inertia = 0.;
        }
    }
    if state.tick % 60 == 0 {
        state.gravity_debounce = false;
    } else {
        state.gravity_debounce = true;
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, state: Res<GameState>) {
    if state.gravity_debounce {
        return;
    }

    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn move_tetromino(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Tetromino>>,
    mut state: ResMut<GameState>,
) {
    let mut tetromino_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 20.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 20.0;
    }

    if direction != 0. && state.key_debounce {
        state.pending_inertia = direction;
        return;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_tetromino_position = tetromino_transform.translation.x + direction;

    tetromino_transform.translation.x = new_tetromino_position.clamp(LEFT_BOUND, RIGHT_BOUND);

    state.key_debounce = true;
}
