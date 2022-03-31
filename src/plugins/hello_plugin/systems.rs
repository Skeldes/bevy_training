use bevy::prelude::*;
use super::components::*;

pub fn add_people(mut commends: Commands) {
    commends.spawn().insert(Person).insert(Name("Elaine Proctor".to_string()));
    commends.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commends.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

pub fn greet_people(time: Res<Time>, mut timer:ResMut<GreetTimer>, query : Query<&super::components::Name, With<Person>>) { //Run on all entity with person and name component 
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {} !", name.0);
        }
    }
}