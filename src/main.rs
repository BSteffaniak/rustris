use bevy::sprite::collide_aabb::collide;
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
                .with_system(ticker)
                .with_system(move_tetromino.after(ticker))
                .with_system(check_for_collisions.after(move_tetromino))
                .with_system(apply_velocity.after(check_for_collisions)),
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
struct Tetromino {
    active: bool,
}

#[derive(Component)]
struct Square;

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
    let squares: [Entity; 4] = [
        commands
            .spawn((
                new_component(INITIAL_TETROMINO_POSITION, TETROMINO_SIZE, TETROMINO_COLOR),
                Square,
            ))
            .id(),
        commands
            .spawn((
                new_component(INITIAL_TETROMINO_POSITION, TETROMINO_SIZE, TETROMINO_COLOR),
                Square,
            ))
            .id(),
        commands
            .spawn((
                new_component(INITIAL_TETROMINO_POSITION, TETROMINO_SIZE, TETROMINO_COLOR),
                Square,
            ))
            .id(),
        commands
            .spawn((
                new_component(INITIAL_TETROMINO_POSITION, TETROMINO_SIZE, TETROMINO_COLOR),
                Square,
            ))
            .id(),
    ];

    commands
        .spawn((
            new_component(INITIAL_TETROMINO_POSITION, TETROMINO_SIZE, TETROMINO_COLOR),
            (
                Tetromino { active: true },
                Velocity(INITIAL_TETROMINO_DIRECTION.normalize() * TETROMINO_BLOCK_SIZE),
                Collider,
            ),
        ))
        .push_children(&squares);
}

fn spawn_board(commands: &mut Commands) {
    // BOTTOM
    commands.spawn((
        new_component(
            Vec2::new(0., -BOARD_HEIGHT / 2. - WALL_THICKNESS / 2.),
            Vec2::new(BOARD_WIDTH + WALL_THICKNESS * 2., WALL_THICKNESS),
            Color::CYAN,
        ),
        (Wall, Collider),
    ));
    // TOP
    commands.spawn((
        new_component(
            Vec2::new(0., BOARD_HEIGHT / 2. + WALL_THICKNESS / 2.),
            Vec2::new(BOARD_WIDTH + WALL_THICKNESS * 2., WALL_THICKNESS),
            Color::CYAN,
        ),
        Wall,
    ));
    // LEFT
    commands.spawn((
        new_component(
            Vec2::new(-BOARD_WIDTH / 2. - WALL_THICKNESS / 2., 0.),
            Vec2::new(WALL_THICKNESS, BOARD_HEIGHT + WALL_THICKNESS * 2.),
            Color::CYAN,
        ),
        (Wall, Collider),
    ));
    // RIGHT
    commands.spawn((
        new_component(
            Vec2::new(BOARD_WIDTH / 2. + WALL_THICKNESS / 2., 0.),
            Vec2::new(WALL_THICKNESS, BOARD_HEIGHT + WALL_THICKNESS * 2.),
            Color::CYAN,
        ),
        (Wall, Collider),
    ));
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

fn apply_velocity(mut query: Query<(&mut Velocity, &mut Transform), With<Tetromino>>) {
    query.for_each_mut(|(mut velocity, mut transform)| {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;

        velocity.x = 0.;
        velocity.y = 0.;
    });
}

fn move_tetromino(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Tetromino), With<Tetromino>>,
    mut state: ResMut<GameState>,
) {
    query.for_each_mut(|(mut tetromino_velocity, tetromino)| {
        if !tetromino.active {
            return;
        }

        if keyboard_input.pressed(KeyCode::Left) && state.key_debounce == 0 {
            tetromino_velocity.x = -TETROMINO_BLOCK_SIZE;
        } else if keyboard_input.just_released(KeyCode::Left) {
            state.key_debounce = 0;
        }

        if keyboard_input.pressed(KeyCode::Right) && state.key_debounce == 0 {
            tetromino_velocity.x = TETROMINO_BLOCK_SIZE;
        } else if keyboard_input.just_released(KeyCode::Right) {
            state.key_debounce = 0;
        }

        if tetromino_velocity.x != 0. && state.key_debounce == 0 {
            // Calculate the new horizontal paddle position based on player input
            state.key_debounce = 14;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            state.tick += 20;
        }
        if !state.gravity_debounce {
            tetromino_velocity.y = -TETROMINO_BLOCK_SIZE;
        }
    });
}

fn check_for_collisions(
    mut commands: Commands,
    mut tetromino_query: Query<
        (Entity, &mut Velocity, &Transform, &mut Tetromino),
        With<Tetromino>,
    >,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    tetromino_query.for_each_mut(
        |(tetromino_entity, mut tetromino_velocity, tetromino_transform, mut tetromino)| {
            if !tetromino.active {
                return;
            }

            for (entity, transform) in &collider_query {
                if entity == tetromino_entity {
                    continue; // Do not check collision with self
                }

                let x_position = tetromino_transform
                    .with_translation(
                        tetromino_transform.translation + Vec3::new(tetromino_velocity.0.x, 0., 0.),
                    )
                    .translation;

                if check_collision(
                    tetromino_transform,
                    transform,
                    &mut collision_events,
                    x_position,
                ) {
                    tetromino_velocity.0.x = 0.;
                }

                let y_position = tetromino_transform
                    .with_translation(
                        tetromino_transform.translation + Vec3::new(0., tetromino_velocity.0.y, 0.),
                    )
                    .translation;

                if check_collision(
                    tetromino_transform,
                    transform,
                    &mut collision_events,
                    y_position,
                ) {
                    tetromino.active = false;
                    tetromino_velocity.0.y = 0.;
                    spawn_tetromino(&mut commands);
                }
            }
        },
    );
}

fn check_collision(
    tetromino_transform: &Transform,
    transform: &Transform,
    collision_events: &mut EventWriter<CollisionEvent>,
    position: Vec3,
) -> bool {
    let tetromino_size = tetromino_transform.scale.truncate();

    let collision = collide(
        position,
        tetromino_size,
        transform.translation,
        transform.scale.truncate(),
    );

    if collision.is_some() {
        // Sends a collision event so that other systems can react to the collision
        collision_events.send_default();

        true
    } else {
        false
    }
}
