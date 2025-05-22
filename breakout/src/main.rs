use bevy::prelude::*;


const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_SPEED: f32 = 500.0;

const GAP_BETWEEN_PADDLE_AND_FLOOR: f32 = 60.0;

const BOTTOM_WALL: f32 = -300.;

const PADDLE_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                move_paddle,
            ).chain(),
        )
        .run();
}

#[derive(Component)]
struct Paddle;

#[derive(Component, Default)]
struct Collider;

fn setup(
    mut commands: Commands,
)
{
    commands.spawn(Camera2d);

        // Paddle
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, paddle_y, 0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        Paddle,
        Collider,
    ));

}

fn move_paddle(
    keyboard_input : Res<ButtonInput<KeyCode>>,
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    time: Res<Time>,
)
{
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft)
    {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight)
    {
        direction += 1.0;
    }

    let new_paddle_position = paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();

    paddle_transform.translation.x = new_paddle_position;

} 
// #[derive(Component)]
// struct Person;

// #[derive(Component)]
// struct Name(String);

// fn main() 
// {
//     App::new()
//         .add_plugins(DefaultPlugins)  
//         .add_plugins(HelloPlugin)
//         .run();      
// }

// fn add_people(mut commands: Commands)
// {
//     commands.spawn((Person, Name("Elaina Proctor".to_string())));
//     commands.spawn((Person, Name("Renzo Hume".to_string())));
//     commands.spawn((Person, Name("Zayne Nieves".to_string())));
// }

// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>)
// {
//     if timer.0.tick(time.delta()).just_finished()
//     {
//         for name in &query
//         {
//             println!("hello {}!", name.0);
//         }
//     }
// }

// fn update_people(mut query: Query<&mut Name, With<Person>>)
// {
//     for mut name in &mut query
//     {
//         if name.0 == "Elaina Proctor"
//         {
//             name.0 = "Elaina Hume".to_string();
//             break;
//         }
//     }
// }

// #[derive(Resource)]
// struct GreetTimer(Timer);

// pub struct HelloPlugin;

// impl Plugin for HelloPlugin 
// {
//     fn build(&self, app: &mut App) 
//     {
//         app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
//         app.add_systems(Startup, add_people);
//         app.add_systems(Update, (update_people,greet_people).chain());    
//     }
// }