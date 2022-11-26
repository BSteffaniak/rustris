use bevy::{prelude::*, time::FixedTimestep};

const TIME_STEP: f32 = 1.0 / 60.;
const TETROMINO_BLOCK_SIZE: f32 = 32.;
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.3, 0.3);
const TETROMINO_COLOR: Color = Color::rgb(0.2, 0.2, 0.9);
const TETROMINO_SIZE: Vec3 = Vec3::new(TETROMINO_BLOCK_SIZE * 3., TETROMINO_BLOCK_SIZE, 0.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0., -1.);
const BOARD_WIDTH: f32 = TETROMINO_BLOCK_SIZE * 12.;
const BOARD_HEIGHT: f32 = TETROMINO_BLOCK_SIZE * 20.;
const LEFT_BOUND: f32 = -BOARD_WIDTH / 2.;
const RIGHT_BOUND: f32 = BOARD_WIDTH / 2.;
const WALL_THICKNESS: f32 = 32.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(window_resize_system)
        .insert_resource(GameState {
            tick: 0,
            key_debounce: 0,
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
    key_debounce: u32,
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

    // TOP
    commands.spawn((
        new_component(
            Vec3::new(0., -BOARD_HEIGHT / 2. - WALL_THICKNESS / 2., 0.),
            Vec3::new(BOARD_WIDTH + WALL_THICKNESS * 2., WALL_THICKNESS, 0.),
            Color::rgb(0.2, 0.2, 0.9),
        ),
        Wall,
    ));
    // BOTTOM
    commands.spawn((
        new_component(
            Vec3::new(0., BOARD_HEIGHT / 2. + WALL_THICKNESS / 2., 0.),
            Vec3::new(BOARD_WIDTH + WALL_THICKNESS * 2., WALL_THICKNESS, 0.),
            Color::rgb(0.2, 0.2, 0.9),
        ),
        Wall,
    ));
    // LEFT
    commands.spawn((
        new_component(
            Vec3::new(-BOARD_WIDTH / 2. - WALL_THICKNESS / 2., 0., 0.),
            Vec3::new(WALL_THICKNESS, BOARD_HEIGHT + WALL_THICKNESS * 2., 0.),
            Color::rgb(0.2, 0.2, 0.9),
        ),
        Wall,
    ));
    // RIGHT
    commands.spawn((
        new_component(
            Vec3::new(BOARD_WIDTH / 2. + WALL_THICKNESS / 2., 0., 0.),
            Vec3::new(WALL_THICKNESS, BOARD_HEIGHT + WALL_THICKNESS * 2., 0.),
            Color::rgb(0.2, 0.2, 0.9),
        ),
        Wall,
    ));
    /*
        commands.spawn((
            new_component(
                Vec3::new(
                    -BOARD_WIDTH / 2. + TETROMINO_SIZE.x / 2.,
                    BOARD_HEIGHT / 2. - TETROMINO_SIZE.y / 2.,
                    0.0,
                ),
                TETROMINO_SIZE,
                TETROMINO_COLOR,
            ),
            Tetromino,
            Velocity(INITIAL_BALL_DIRECTION.normalize() * TETROMINO_BLOCK_SIZE),
        ));
    */
    spawn_component(
        commands,
        Vec3::new(
            BOARD_WIDTH / 2. - TETROMINO_SIZE.x / 2.,
            BOARD_HEIGHT / 2. - TETROMINO_SIZE.y / 2.,
            0.0,
        ),
        TETROMINO_SIZE,
        TETROMINO_COLOR,
        (
            Tetromino,
            Velocity(INITIAL_BALL_DIRECTION.normalize() * TETROMINO_BLOCK_SIZE),
        ),
    );
}

fn spawn_component<T: Bundle>(
    mut commands: Commands,
    translation: Vec3,
    scale: Vec3,
    color: Color,
    component: T,
) {
    commands.spawn((new_component(translation, scale, color), component));
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

fn ticker(mut state: ResMut<GameState>) {
    state.tick = (state.tick + 1) % 1_000_000;

    if state.key_debounce > 0 {
        state.key_debounce -= 1;
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
        direction -= TETROMINO_BLOCK_SIZE;
    } else if keyboard_input.just_released(KeyCode::Left) {
        state.key_debounce = 0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += TETROMINO_BLOCK_SIZE;
    } else if keyboard_input.just_released(KeyCode::Right) {
        state.key_debounce = 0;
    }

    if direction != 0. && state.key_debounce == 0 {
        // Calculate the new horizontal paddle position based on player input
        let new_tetromino_position = tetromino_transform.translation.x + direction;

        tetromino_transform.translation.x = new_tetromino_position.clamp(
            LEFT_BOUND + TETROMINO_SIZE.x / 2.,
            RIGHT_BOUND - TETROMINO_SIZE.x / 2.,
        );

        state.key_debounce = 14;
    }
}
