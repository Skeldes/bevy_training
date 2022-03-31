mod components;
mod systems;

use bevy::prelude::*;

use systems::*;
use components::*;


pub struct HelloPlugin;

impl Plugin for HelloPlugin{
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people)//startup system run exactly once before any other system
            .add_system(greet_people);
    }
}