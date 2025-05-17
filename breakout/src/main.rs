use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() 
{
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}

fn add_people(mut commands: Commands)
{
    commands.spawn((Person, Name("Elaine Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayne Nieves".to_string())));
}

fn hello_world()
{
    println!("Hello World");
}

fn greet_people(query: Query<&Name, With<Person>>)
{
    for name in &query
    {
        println!("hello {}!", name.0);
    }
}