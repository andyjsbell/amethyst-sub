use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings, VirtualKeyCode};

use crate::{map::{Coordinate, Map, TileType, coordinate_to_grid}, state::Player};

#[derive(SystemDesc)]
pub struct MovementSystem;

#[derive(PartialEq)]
enum Direction {
    None,
    Left,
    Right,
    Up,
    Down,
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Map>,
    );

    fn run(&mut self, (mut transforms, players, input, map): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {
            let mut dir = Direction::None;
            
            if input.key_is_down(VirtualKeyCode::Left) {
                dir = Direction::Left;
            } else if input.key_is_down(VirtualKeyCode::Right) {
                dir = Direction::Right;
            } else if input.key_is_down(VirtualKeyCode::Up) {
                dir = Direction::Up;
            } else if input.key_is_down(VirtualKeyCode::Down) {
                dir = Direction::Down;
            } 
            
            let mut next_x = transform.translation().x;
            let mut next_y = transform.translation().y;
            let mut offset_x = 0.0;
            let mut offset_y = 0.0;
            
            match dir {
                Direction::None => {}
                Direction::Left => { 
                    next_x -= 1.0 * player.speed;
                    offset_x = -8.0;
                }
                Direction::Right => {
                    next_x += 1.0 * player.speed;
                    offset_x = 8.0;
                }
                Direction::Up => {
                    next_y += 1.0 * player.speed;
                    offset_y = 12.0;
                }
                Direction::Down => {
                    next_y -= 1.0 * player.speed;
                    offset_y = -12.0;
                }
            }

            let grid = coordinate_to_grid(Coordinate(next_x + offset_x, next_y + offset_y));
            let idx = map.grid_to_index(grid);
            if map.tiles.len() > idx && map.tiles[idx] == TileType::Floor {
                transform.set_translation_x(next_x);    
                transform.set_translation_y(next_y);
            }
        }
    }

}
