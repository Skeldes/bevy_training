mod plugins;

use bevy::prelude::*;
use plugins::hello_plugin::*;



#[derive(Default)]
struct Square{
    color: Color,
    x: usize,
    y: usize,
}




fn setup(mut commands: Commands){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle{
        sprite : Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..Default::default()
        },
        ..Default::default()
    });
}

// fn move_sprite(time: Res<Time>,mut timer: ResMut<GreetTimer>, mut query: Query<&Sprite>){
//     if timer.0.tick(time.delta()).just_finished() {
//         for sprite in query.iter(){
//             sprite.
//         }
//     }
// }

fn main() {
    App::new()
    .insert_resource(WindowDescriptor {
        title: "Bevy training !".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    //.add_startup_system(setup)
    .run();
}