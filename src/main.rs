use bevy::core::FixedTimestep;
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 500.,
            height: 500.,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_input)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0))
                .with_system(food_spawner),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(5.0))
                .with_system(snake_movement)
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(size_scaling)
                .with_system(position_translation),
        )
        .add_plugins(DefaultPlugins)
        .run();
        }

// Setup Camera   [2022-07-21 Thu]                                     :Code:

// [[file:../literate-snake.org::*Setup Camera \[2022-07-21 Thu\]][Setup Camera   [2022-07-21 Thu]:1]]
fn setup_camera(mut commands: Commands){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
// Setup Camera   [2022-07-21 Thu]:1 ends here

// Create Snake   [2022-07-21 Thu]                                     :Code:

// [[file:../literate-snake.org::*Create Snake \[2022-07-21 Thu\]][Create Snake   [2022-07-21 Thu]:1]]
#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
enum SnakeDirection{
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8))
        .insert(SnakeDirection::UP);
}
// Create Snake   [2022-07-21 Thu]:1 ends here

// Control Snake   [2022-07-21 Thu]                                    :Code:

// [[file:../literate-snake.org::*Control Snake \[2022-07-21 Thu\]][Control Snake   [2022-07-21 Thu]:1]]
fn snake_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut SnakeDirection, With<SnakeHead>>,
) {
    for mut dir in head_positions.iter_mut() {
        match keyboard_input.get_pressed().next() {
            Some(KeyCode::Left) => match *dir {
                SnakeDirection::UP | SnakeDirection::DOWN => *dir = SnakeDirection::LEFT,
                _ => {}
            },
            Some(KeyCode::Right) => match *dir {
                SnakeDirection::UP | SnakeDirection::DOWN => *dir = SnakeDirection::RIGHT,
                _ => {}
            },
            Some(KeyCode::Up) => match *dir {
                SnakeDirection::LEFT | SnakeDirection::RIGHT => *dir = SnakeDirection::UP,
                _ => {}
            },
            Some(KeyCode::Down) => match *dir {
                SnakeDirection::LEFT | SnakeDirection::RIGHT => *dir = SnakeDirection::DOWN,
                _ => {}
            },
            _ => {}
        }
    }
}

fn snake_movement(
    mut head_positions: Query<(&mut Position, &SnakeDirection), With<SnakeHead>>,
) {
    for (mut pos, dir) in head_positions.iter_mut(){
        match dir{
            SnakeDirection::UP => pos.y += 1,
            SnakeDirection::DOWN => pos.y -= 1,
            SnakeDirection::LEFT => pos.x -= 1,
            SnakeDirection::RIGHT => pos.x += 1,
        }
    }
}
// Control Snake   [2022-07-21 Thu]:1 ends here

// Grid   [2022-07-21 Thu]                                             :Code:

// [[file:../literate-snake.org::*Grid \[2022-07-21 Thu\]][Grid   [2022-07-21 Thu]:1]]
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}
// Grid   [2022-07-21 Thu]:1 ends here

// Food   [2022-07-22 Fri]                                             :Code:

// [[file:../literate-snake.org::*Food \[2022-07-22 Fri\]][Food   [2022-07-22 Fri]:1]]
const FOOD_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

#[derive(Component)]
struct Food;

fn food_spawner(mut commands: Commands, food_query: Query<&Food>) {
    //If we already have a food on the screen, don't create another.
    if let Ok(_) = food_query.get_single() { return; }

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
}

fn eat_food(mut commands: Commands, positions_query: Query<&Position>){}
// Food   [2022-07-22 Fri]:1 ends here
