use bevy::prelude::*;

struct GreetTimer(Timer);
pub struct HelloPlugin;

impl Plugin for HelloPlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)//startup system run exactly once before any other system
            .add_system(greet_people);
    }
}


#[derive(Component)]
struct Name(String);


#[derive(Component)]
struct Person;



fn add_people(mut commends: Commands) {
    commends.spawn().insert(Person).insert(Name("Elaine Proctor".to_string()));
    commends.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commends.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(time: Res<Time>, mut timer:ResMut<GreetTimer>, query : Query<&Name, With<Person>>) { //Run on all entity with person and name component 
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {} !", name.0);
        }
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin) 
    .run();
}
