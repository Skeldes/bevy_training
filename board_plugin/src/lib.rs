pub mod components;
pub mod ressources;

use bevy::{
    log,
    prelude::*,
};

use ressources::{
    board_options::BoardOptions,
    tile_map::TileMap,
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);//un system est une fonction qui est lancé à chaque boucle de jeu, un system_startup n'est lancé qu'au lancement
        log::info!("Loaded Board Plugin !");
    }
}


impl BoardPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Option<Res<WindowDescriptor>>
    ) {
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),        
        };
        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
    }

    fn adaptative_tile_size(
        window: Option<Res<WindowDescriptor>>,
        (min, max): (f32, f32),
        (width, height): (u16, u16)
    ) -> f32 {
        let window = match window {
            None => WindowDescriptor::default(),
            Some(o) => o.clone(),
        };
        let max_width = window.width / width as f32;
        let max_height = window.height / height as f32;
        max_width.min(max_height).clamp(min, max)
    }
}