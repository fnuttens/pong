use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{
        Join,
        Read,
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
    Ball,
    ScoreBoard,
    ScoreText,
    ARENA_WIDTH,
    ARENA_HEIGHT,
};
use crate::audio;

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, audio::Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (mut balls, mut locals, mut ui_text, mut scores, score_text, storage, sounds, audio_output): Self::SystemData
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
                audio::play_score_sound(&*sounds, &storage, audio_output.as_deref());
                println!("Score: | {:^3} | {:^3} |", scores.score_left, scores.score_right);
            }
        }
    }
}
