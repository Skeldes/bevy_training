mod plugins;

use bevy::prelude::*;
//use plugins::hello_plugin::*;
use board_plugin::{
    BoardPlugin,
    BoardOptions,
};

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;



#[derive(Default)]
struct _Square{
    _color: Color,
    _x: usize,
    _y: usize,
}




fn _setup(mut commands: Commands){
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


fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "Bevy training !".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins);
    app.insert_resource(BoardOptions {
        map_size: (20,20),
        bomb_count: 40,
        tile_padding: 3.0,
        ..Default::default()
    });
    app.add_plugin(BoardPlugin);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    //.add_plugin(HelloPlugin)
    //.add_startup_system(setup)
    app.add_startup_system(camera_setup)
    .run();
}