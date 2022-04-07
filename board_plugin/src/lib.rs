pub mod components;
pub mod ressources;

use bevy::{
    log,
    prelude::*,
};

use ressources::tile_map::TileMap;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board);//un system est une fonction qui est lancé à chaque boucle de jeu, un system_startup n'est lancé qu'au lancement
        log::info!("Loaded Board Plugin !");
    }
}


impl BoardPlugin {
    pub fn create_board() {
        let mut tile_map = TileMap::empty(20, 20);
        tile_map.set_bombs(40);
        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
    }
}