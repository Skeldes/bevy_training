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
}