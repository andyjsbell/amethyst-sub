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
        for (_, transform) in (&senseis, &mut transforms).join() {
            let movement_x = input.axis_value("move_x");
            let movement_y = input.axis_value("move_y");
            if let Some(x) = movement_x {
                transform.set_translation_x(transform.translation().x + (x * 1.2));    
            }

            if let Some(y) = movement_y {
                transform.set_translation_y(transform.translation().y + (y * 1.2));
            }
        }
    }
}
