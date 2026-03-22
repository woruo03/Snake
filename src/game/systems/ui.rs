use bevy::prelude::*;

use crate::game::{
    components::{HighScoreText, ScoreText},
    messages::ScoreChangedMsg,
    resources::GameStatus,
};

pub(crate) fn apply_score_changed_ui_system(
    mut reader: MessageReader<ScoreChangedMsg>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<HighScoreText>)>,
    mut high_query: Query<&mut Text, (With<HighScoreText>, Without<ScoreText>)>,
    status: Res<GameStatus>,
) {
    let mut newest: Option<u32> = None;
    for msg in reader.read() {
        newest = Some(msg.score);
    }

    let Some(score) = newest else {
        return;
    };

    if let Ok(mut score_text) = score_query.single_mut() {
        score_text.0 = format!("Score: {score}");
    }

    if let Ok(mut high_text) = high_query.single_mut() {
        high_text.0 = format!("High Score: {}", status.high_score);
    }
}
