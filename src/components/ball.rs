use amethyst::ecs::prelude::{Component, DenseVecStorage};

use crate::pong::{
    BALL_RADIUS,
    BALL_VELOCITY_X,
    BALL_VELOCITY_Y
};

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
            radius: BALL_RADIUS,
        }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
