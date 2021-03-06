use amethyst::{
    core::Transform,
    ecs::prelude::{Read, Join, ReadStorage, System, WriteStorage},
};
use crate::components::{Ball, Paddle, Side};
use crate::resources::Arena;

pub struct BounceBallSystem;

impl<'s> System<'s> for BounceBallSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
        Read<'s, Arena>,
    );

    fn run(&mut self, (mut balls, paddles, transforms, arena): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            let collides_with_top = ball_y <= ball.radius && ball.velocity[1] < 0.0;
            let collides_with_bottom = ball_y >= arena.height - ball.radius && ball.velocity[1] > 0.0;

            if collides_with_top || collides_with_bottom
            {
                ball.velocity[1] = -ball.velocity[1];
            }

            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - (paddle.width * 0.5);
                let paddle_y = paddle_transform.translation().y - (paddle.height * 0.5);

                if point_inside_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius,
                    paddle_y - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius,
                ) {
                    if (paddle.side == Side::Left && ball.velocity[0] < 0.0)
                        || (paddle.side == Side::Right && ball.velocity[0] > 0.0)
                    {
                        ball.velocity[0] = -ball.velocity[0];
                    }
                }
            }
        }
    }
}

fn point_inside_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}