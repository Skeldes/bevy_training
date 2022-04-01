use crate::ressources::tile::Tile;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct TileMap{
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn empty(width: u16, height: u16) -> Self { //constructor (more or less)
        let map = (0..height)//crée un vecteur de taille height
            .into_iter() //parcourt chaque élément
            .map(|_| (0..width).into_iter().map(|_| Tile::Empty).collect())
            .collect();
        self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    #[cfg(featurte = "debug")]
    pub fn console_output(&self) -> String {
        let mut bufffer = format!(
            "Map ({},{}) with {} bombs:\n",
            self.width, self.height, self.bomb_count
        );
        let line: String = (0..=(self.width+1)).into_iter().map(|_| '-').collect();
        buffer = format!("{}{}\n", buffer, line);
        for line in self.iter().rev(){
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n",buffer);
        }
        buffer = format!("{}{}", buffer, line)
    }



    //Getter
    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }
}



impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}


impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}