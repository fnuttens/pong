use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::{
        Join,
        ReadStorage,
        System,
        SystemData,
        World,
        WriteStorage,
    },
};

use crate::pong::{
    ARENA_HEIGHT,
    components::{
        Ball,
        Side,
        Paddle,
    }
};

pub struct BounceSystem;

impl<'s> System<'s> for BounceSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Paddle>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, paddles, transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &transforms).join() {
            let ball_x = transform.translation().x;
            let ball_y = transform.translation().y;

            let is_ball_at_bottom_and_descending = ball_y <= ball.radius && ball.velocity[1] < 0.0;
            let is_ball_at_top_and_ascending = ball_y >= ARENA_HEIGHT - ball.radius && ball.velocity[1] > 0.0;

            if is_ball_at_bottom_and_descending || is_ball_at_top_and_ascending {
                ball.velocity[1] = -ball.velocity[1];
            }

            for (paddle, paddle_transform) in (&paddles, &transforms).join() {
                let paddle_x = paddle_transform.translation().x - (paddle.width * 0.5);
                let paddle_y = paddle_transform.translation().y - (paddle.height * 0.5);

                if is_point_in_rect(
                    ball_x,
                    ball_y,
                    paddle_x - ball.radius,
                    paddle_y - ball.radius,
                    paddle_x + paddle.width + ball.radius,
                    paddle_y + paddle.height + ball.radius) {
                    let is_left_paddle_and_ball_going_left = paddle.side == Side::Left && ball.velocity[0] < 0.0;
                    let is_right_paddle_and_ball_going_right = paddle.side == Side::Right && ball.velocity[0] > 0.0;

                    if is_left_paddle_and_ball_going_left || is_right_paddle_and_ball_going_right {
                        ball.velocity[0] = -ball.velocity[0];
                    }
                }
            }
        }
    }
}

fn is_point_in_rect(
    x: f32,
    y: f32,
    left: f32,
    bottom: f32,
    right: f32,
    top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
