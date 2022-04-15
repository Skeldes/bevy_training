mod bounds;
mod events;
mod ressources;
mod systems;
pub mod components;

use bevy::{
    log,
    math::Vec3Swizzles,
    prelude::*, utils::{HashMap, AHashExt},
};
use crate::{
    components::{
        BombNeighbor,
        Bomb,
        Coordinates,
        Uncover,
    },
    ressources::tile::Tile,
    tile_map::TileMap,
};

pub use bounds::*;
pub use ressources::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::create_board)//un system est une fonction qui est lancé à chaque boucle de jeu, un system_startup n'est lancé qu'au lancement
            .add_system(systems::input::input_handling);
        log::info!("Loaded Board Plugin !");


        #[cfg(feature = "debug")]
        {
            // app.register_inspectable::<Coordinates>()
            //     .register_inspectable::<BombNeighbor>()
            //     .register_inspectable::<Bomb>()
            //     .register_inspectable::<Uncover>();
        }
    }
}

impl BoardPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        window: Option<Res<WindowDescriptor>>,
        asset_server: Res<AssetServer>
    ) {
        let bomb_image = asset_server.load("sprites/bomb.png");
        let font = asset_server.load("fonts/Nightmare.ttf");
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),        
        };

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);


        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => v,
            TileSize::Adaptive {min, max} => Self::adaptative_tile_size(
                window,
                (min, max),
                (tile_map.width(), tile_map.height()) 
            ),
        };

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size
        );

        log::info!("Board size: {}", board_size);

        // define the board anchor position
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            },
            BoardPosition::Custom(p) => p,
        };

        let mut covered_tiles = HashMap::with_capacity((tile_map.width() * tile_map.height()).into());

        commands
            .spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                    Self::spawn_tiles(
                        parent,
                        &tile_map,
                        tile_size,
                        options.tile_padding,
                        Color::GRAY,
                        bomb_image,
                        font,
                        Color::DARK_GRAY,
                        &mut covered_tiles
                    )
            });
        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
        });
        //#[cfg(feature = "debug")]
        //log::info!("{}", tile_map.console_output());
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

    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, size: f32) -> Text2dBundle {
        let (text, color) = (
            count.to_string(),
            match count {
                1 => Color::WHITE,
                2 => Color::GREEN,
                3 => Color::YELLOW,
                4 => Color::ORANGE,
                _ => Color::PURPLE,
            }
        );
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: text,
                    style: TextStyle {
                        color,
                        font,
                        font_size: size
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center
                },
            },
            transform: Transform::from_xyz(0.,0.,1.),
            ..Default::default()
        }
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        color: Color,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
        covered_tile_color: Color,
        covered_tiles: &mut HashMap<Coordinates, Entity>
    ){
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };
                let mut cmd = parent.spawn();
                cmd.insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::splat(size- padding)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
                    ..Default::default()
                })
                .insert(Name::new(format!("Tile({}, {})", x, y)))
                .insert(coordinates);
                cmd.with_children(|parent| {
                    let entity = parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(size - padding)),
                                color: covered_tile_color,
                                ..Default::default()
                            },
                            transform : Transform::from_xyz(0., 0., 2.),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);
                });
                match tile {
                    Tile::Bomb => {
                        cmd.insert(Bomb);
                        cmd.with_children(|parent| {
                            parent.spawn_bundle(SpriteBundle {
                                sprite: Sprite{
                                    custom_size: Some(Vec2::splat(size - padding)),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0., 1.),
                                texture: bomb_image.clone(),
                                ..Default::default()
                            });
                        });
                    },
                    Tile::BombNeighbor(v) => {
                        cmd.insert(BombNeighbor {count : *v});
                        cmd.with_children(|parent| {
                            parent.spawn_bundle(Self::bomb_count_text_bundle(
                                *v,
                                font.clone(),
                                size - padding
                            ));
                        });
                    }
                    Tile::Empty => (),
                }
            }
        }
    }
}