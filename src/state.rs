use std::fmt::Debug;

use amethyst::{assets::{AssetStorage, Loader, Handle}, core::transform::Transform, ecs::{Component, DenseVecStorage}, input::{get_key, is_close_requested, is_key_down, VirtualKeyCode}, prelude::*, renderer::{Camera, ImageFormat, Sprite, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture}, ui::{
        Anchor, FontHandle, LineMode, Stretch, TtfFormat, UiButtonBuilder, UiImage, UiText,
        UiTransform,
    }, window::ScreenDimensions};
use map::{TileType, create_simple_map};

use crate::map;
// use log::info;
pub struct Sensei;

#[derive(Debug)]
pub struct Block;

const START_POS: (usize, usize) = (10, 8);

impl Component for Sensei {
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
        
        world.register::<Sensei>();
        world.register::<Block>();
        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        let sprites = load_sprites(world);
        initialise_sensei(world, sprites.clone());
        initialise_map(world, &dimensions, sprites);
        // init_sprites(world, &sprites, &dimensions);

        // create_ui_example(world);
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
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

fn initialise_sensei(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        (START_POS.0 * map::GRID_SIZE + map::GRID_SIZE / 2) as f32, 
        (START_POS.1 * map::GRID_SIZE + map::GRID_SIZE / 2) as f32,
        0.0);

    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    world
        .create_entity()
        .with(sprite_render)
        .with(Sensei {})
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

pub fn create_ui_example(world: &mut World) {
    // this creates the simple gray background UI element.
    let ui_background = world
        .create_entity()
        .with(UiImage::SolidColor([0.6, 0.1, 0.2, 1.0]))
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            30.0,
            -30.,
            0.,
            250.,
            50.,
        ))
        .build();

    // This simply loads a font from the asset folder and puts it in the world as a resource,
    // we also get a ref to the font that we then can pass to the text label we crate later.
    let font: FontHandle = world.read_resource::<Loader>().load(
        "fonts/Bangers-Regular.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    // This creates the actual label and places it on the screen.
    // Take note of the z position given, this ensures the label gets rendered above the background UI element.
    world
        .create_entity()
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            40.0,
            -40.,
            1.,
            200.,
            50.,
        ))
        .with(UiText::new(
            font,
            "Substrate mining for Amethyst".to_string(),
            [1., 1., 1., 1.],
            30.,
            LineMode::Single,
            Anchor::TopLeft,
        ))
        .build();
}
