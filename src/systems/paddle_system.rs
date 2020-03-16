use amethyst::core::Transform;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use crate::components::{Side, Paddle, PADDLE_HEIGHT};
use crate::resources::Arena;

#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Arena>,
    );

    fn run(&mut self, (mut transforms, paddles, input, arena): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                let paddle_y = transform.translation().y;
                let paddle_movement_clamped = (paddle_y + scaled_amount)
                    .min(arena.height - PADDLE_HEIGHT * 0.5)
                    .max(PADDLE_HEIGHT * 0.5);

                transform.set_translation_y(paddle_movement_clamped);
            }
        }
    }
}