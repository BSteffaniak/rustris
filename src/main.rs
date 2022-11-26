use bevy::{prelude::*, time::FixedTimestep};

const TIME_STEP: f32 = 1.0 / 60.;
const TETROMINO_BLOCK_SIZE: f32 = 32.;
const BACKGROUND_COLOR: Color = Color::BLACK;
const TETROMINO_COLOR: Color = Color::CYAN;
const TETROMINO_SIZE: Vec2 = Vec2::new(TETROMINO_BLOCK_SIZE * 1., TETROMINO_BLOCK_SIZE * 4.);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0., -1.);
const INITIAL_TETROMINO_POSITION: Vec2 = Vec2::new(
    -TETROMINO_SIZE.x / 2.,
    BOARD_HEIGHT / 2. - TETROMINO_SIZE.y / 2.,
);
const BOARD_WIDTH: f32 = TETROMINO_BLOCK_SIZE * 10.;
const BOARD_HEIGHT: f32 = TETROMINO_BLOCK_SIZE * 20.;
const LEFT_BOUND: f32 = -BOARD_WIDTH / 2.;
const RIGHT_BOUND: f32 = BOARD_WIDTH / 2.;
const WALL_THICKNESS: f32 = 7.;

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

    spawn_board(&mut commands);

    spawn_component(
        &mut commands,
        INITIAL_TETROMINO_POSITION,
        TETROMINO_SIZE,
        TETROMINO_COLOR,
        (
            Tetromino,
            Velocity(INITIAL_BALL_DIRECTION.normalize() * TETROMINO_BLOCK_SIZE),
        ),
    );
}

fn spawn_board(commands: &mut Commands) {
    // TOP
    spawn_component(
        commands,
        Vec2::new(0., -BOARD_HEIGHT / 2. - WALL_THICKNESS / 2.),
        Vec2::new(BOARD_WIDTH + WALL_THICKNESS * 2., WALL_THICKNESS),
        Color::CYAN,
        Wall,
    );
    // BOTTOM
    spawn_component(
        commands,
        Vec2::new(0., BOARD_HEIGHT / 2. + WALL_THICKNESS / 2.),
        Vec2::new(BOARD_WIDTH + WALL_THICKNESS * 2., WALL_THICKNESS),
        Color::CYAN,
        Wall,
    );
    // LEFT
    spawn_component(
        commands,
        Vec2::new(-BOARD_WIDTH / 2. - WALL_THICKNESS / 2., 0.),
        Vec2::new(WALL_THICKNESS, BOARD_HEIGHT + WALL_THICKNESS * 2.),
        Color::CYAN,
        Wall,
    );
    // RIGHT
    spawn_component(
        commands,
        Vec2::new(BOARD_WIDTH / 2. + WALL_THICKNESS / 2., 0.),
        Vec2::new(WALL_THICKNESS, BOARD_HEIGHT + WALL_THICKNESS * 2.),
        Color::CYAN,
        Wall,
    );
}

fn spawn_component<T: Bundle>(
    commands: &mut Commands,
    translation: Vec2,
    scale: Vec2,
    color: Color,
    component: T,
) {
    commands.spawn((new_component(translation, scale, color), component));
}

fn new_component(translation: Vec2, scale: Vec2, color: Color) -> SpriteBundle {
    SpriteBundle {
        transform: Transform {
            translation: translation.extend(0.),
            scale: scale.extend(0.),
            ..default()
        },
        sprite: Sprite { color, ..default() },
        ..default()
    }
}

fn window_resize_system(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        println!("Window size was: {},{}", window.width(), window.height());
        window.set_resolution(500., 800.);
    } else {
        println!("Could not get window!");
    }
}

fn ticker(mut state: ResMut<GameState>) {
    state.tick += 1;

    if state.key_debounce > 0 {
        state.key_debounce -= 1;
    }

    if state.tick >= 60 {
        state.gravity_debounce = false;
        state.tick = 0;
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

    if keyboard_input.pressed(KeyCode::Down) {
        state.tick += 20;
    }
}
