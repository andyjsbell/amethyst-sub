use std::fmt::Debug;

use amethyst::{assets::{AssetStorage, Loader, Handle}, core::transform::Transform, ecs::{Component, DenseVecStorage}, input::{get_key, is_close_requested, is_key_down, VirtualKeyCode}, prelude::*, renderer::{Camera, ImageFormat, Sprite, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture}, window::ScreenDimensions};
use map::{TileType, create_simple_map};

use crate::map;
// use log::info;
pub struct Player {
    pub speed: f32,
}

#[derive(Debug)]
pub struct Block;

const START_POS: (usize, usize) = (10, 8);

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

/// Our substrate mining game state
pub struct SubState;

impl SimpleState for SubState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        
        world.register::<Player>();
        world.register::<Block>();
        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        let sprites = load_sprites(world);
        initialise_player(world, sprites.clone());
        initialise_map(world, &dimensions, sprites);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        // Keep going
        Trans::None
    }
}

fn initialise_map(world: &mut World, dimensions: &ScreenDimensions, sprite_sheet_handle: Handle<SpriteSheet>) {
    let simple_map = create_simple_map(
        dimensions.width() as usize, 
        dimensions.height() as usize, 
        80, START_POS)
        .unwrap();

            
    world.insert(simple_map.clone());

    let mut row = 0;
    let mut column = 0;
    
    for tile in simple_map.tiles {
        if tile == TileType::Wall {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                ((column * map::GRID_SIZE) + (map::GRID_SIZE / 2)) as f32, 
                ((row * map::GRID_SIZE) + (map::GRID_SIZE / 2)) as f32,
                0.0);
            initialise_block(world, transform, sprite_sheet_handle.clone());
        }

        column += 1;
        if column == simple_map.columns {
            column = 0;
            row += 1;
        }
    }
}

fn initialise_block(world: &mut World, transform: Transform, sprite_sheet_handle: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(sprite_render)
        .with(Block {})
        .with(transform)
        .build();
}

fn initialise_player(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        (START_POS.0 * map::GRID_SIZE + map::GRID_SIZE / 2) as f32, 
        (START_POS.1 * map::GRID_SIZE + map::GRID_SIZE / 2) as f32,
        0.0);

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    world
        .create_entity()
        .with(sprite_render)
        .with(Player {
            speed: 1.2
        })
        .with(transform)
        .build();
}
/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World) -> Handle<SpriteSheet> {

    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/tilesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/tilesheet.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    sheet_handle
}
