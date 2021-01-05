use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::state::Sensei;

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Sensei>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, senseis, input): Self::SystemData) {
        for (sensei, transform) in (&senseis, &mut transforms).join() {
            let movement_x = input.axis_value("");
            let movement_y = input.axis_value("");
            if let Some(x) = movement_x {
                println!("Movement x: {}", x);
            }

            if let Some(y) = movement_y {
                println!("Movement y: {}", y);
            }
        }
    }
}
