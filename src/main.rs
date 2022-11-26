use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::{prelude::*, time::FixedTimestep};

const TIME_STEP: f32 = 1.0 / 60.;
const TETROMINO_BLOCK_SIZE: f32 = 32.;
const BACKGROUND_COLOR: Color = Color::BLACK;
const TETROMINO_COLOR: Color = Color::CYAN;
const TETROMINO_SIZE: Vec2 = Vec2::new(TETROMINO_BLOCK_SIZE * 1., TETROMINO_BLOCK_SIZE * 4.);
const INITIAL_TETROMINO_DIRECTION: Vec2 = Vec2::new(0., -1.);
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
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
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

#[derive(Component)]
struct Collider;

#[derive(Default)]
struct CollisionEvent;

// Add the game's entities to our world
fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    spawn_board(&mut commands);
    spawn_tetromino(&mut commands);
}

fn spawn_tetromino(commands: &mut Commands) {
    spawn_component(
        commands,
        INITIAL_TETROMINO_POSITION,
        TETROMINO_SIZE,
        TETROMINO_COLOR,
        (
            Tetromino,
            Velocity(INITIAL_TETROMINO_DIRECTION.normalize() * TETROMINO_BLOCK_SIZE),
        ),
    );
}

fn spawn_board(commands: &mut Commands) {
    // BOTTOM
    spawn_component(
        commands,
        Vec2::new(0., -BOARD_HEIGHT / 2. - WALL_THICKNESS / 2.),
        Vec2::new(BOARD_WIDTH + WALL_THICKNESS * 2., WALL_THICKNESS),
        Color::CYAN,
        (Wall, Collider),
    );
    // TOP
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
        (Wall, Collider),
    );
    // RIGHT
    spawn_component(
        commands,
        Vec2::new(BOARD_WIDTH / 2. + WALL_THICKNESS / 2., 0.),
        Vec2::new(WALL_THICKNESS, BOARD_HEIGHT + WALL_THICKNESS * 2.),
        Color::CYAN,
        (Wall, Collider),
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
    mut query: Query<(&mut Velocity, &mut Transform), With<Tetromino>>,
    mut state: ResMut<GameState>,
) {
    query.for_each_mut(|(tetromino_velocity, mut tetromino_transform)| {
        if tetromino_velocity.y == 0. {
            return;
        }

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
    });
}

fn check_for_collisions(
    mut commands: Commands,
    mut tetromino_query: Query<(&mut Velocity, &Transform), With<Tetromino>>,
    collider_query: Query<&Transform, With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    tetromino_query.for_each_mut(|(mut tetromino_velocity, tetromino_transform)| {
        if tetromino_velocity.y == 0. {
            return;
        }

        let tetromino_size = tetromino_transform.scale.truncate();

        // check collision with walls
        for transform in &collider_query {
            let next_position = tetromino_transform
                .with_translation(
                    tetromino_transform.translation - Vec3::new(0., TETROMINO_BLOCK_SIZE, 0.),
                )
                .translation;

            let collision = collide(
                next_position,
                tetromino_size,
                transform.translation,
                transform.scale.truncate(),
            );

            if let Some(collision) = collision {
                // Sends a collision event so that other systems can react to the collision
                collision_events.send_default();

                let mut hit_bottom = false;

                match collision {
                    Collision::Left => { /* do nothing */ }
                    Collision::Right => { /* do nothing */ }
                    Collision::Top => hit_bottom = true,
                    Collision::Bottom => hit_bottom = true,
                    Collision::Inside => hit_bottom = true,
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if hit_bottom {
                    tetromino_velocity.y = 0.;
                    spawn_tetromino(&mut commands);
                }
            }
        }
    });
}
