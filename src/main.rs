use bevy::prelude::*;
use color_eyre::eyre::Result;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String, String);

fn setup_people(mut commands: Commands) {
    commands.spawn((Person, Name("Ntombizodwa".to_owned(), "Cartwright".to_owned())));
    commands.spawn((Person, Name("Melantha".to_owned(), "Steffen".to_owned())));
    commands.spawn((Person, Name("Larysa".to_owned(), "Ćosić".to_owned())));
    commands.spawn((Person, Name("Arati".to_owned(), "Bélanger".to_owned())));
}

fn hello_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello, {} {}!", name.0, name.1);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.1 == "Cartwright" {
            name.1 = "Ćosić".to_owned();
            break;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_people)
            .add_systems(Update, (hello_people, update_people, hello_people).chain());
    }
}

fn main() -> Result<()> {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();

    Ok(())
}
