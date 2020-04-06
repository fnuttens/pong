use amethyst::ecs::prelude::Entity;

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: u32,
    pub score_right: u32,
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}
