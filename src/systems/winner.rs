use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{
        Join,
        ReadExpect,
        System,
        SystemData,
        World,
        Write,
        WriteStorage,
    },
    ui::UiText,
};


use crate::pong::{
    ARENA_WIDTH,
    ARENA_HEIGHT,
    components::{
        Ball,
        ScoreBoard,
        ScoreText,
    }
};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(
        &mut self,
        (mut balls, mut locals, mut ui_text, mut scores, score_text): Self::SystemData
    ) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let right_player_scored = ball_x <= ball.radius;
            let left_player_scored = ball_x >= ARENA_WIDTH - ball.radius;

            if right_player_scored {
                scores.score_right = (scores.score_right + 1)
                    .min(999);

                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
            } else if left_player_scored {
                scores.score_left = (scores.score_left + 1)
                    .min(999);

                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
            }

            if right_player_scored || left_player_scored {
                ball.velocity[0] = -ball.velocity[0];
                transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);
                println!("Score: | {:^3} | {:^3} |", scores.score_left, scores.score_right);
            }
        }
    }
}
