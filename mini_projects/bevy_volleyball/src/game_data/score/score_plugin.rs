/*
Copyright (©) 2025 Thinh Truong Nguyen

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without limitation in the rights to use, copy, modify, merge,
publish, and/ or distribute copies of the Software in an educational or
personal context, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

Permission is granted to sell and/ or distribute copies of the Software in
a commercial context, subject to the following conditions:
Substantial changes: adding, removing, or modifying large parts, shall be
developed in the Software. Reorganizing logic in the software does not warrant
a substantial change.

This project and source code may use libraries or frameworks that are
released under various Open-Source licenses. Use of those libraries and
frameworks are governed by their own individual licenses.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

Created At: 01:52 - 09/12/2025
*/
use crate::game_data::score::Score;
use crate::game_objects::ball::Ball;
use crate::game_util::constants::{BALL_SIZE, BLOB_SOUND, FOREGROUND_HEIGHT, SCORE_SOUND};
use crate::game_util::space_factors::size_factor;
use bevy::app::{App, Plugin};
use bevy::asset::{AssetServer, Handle};
use bevy::audio::{AudioPlayer, AudioSource, PlaybackSettings, Volume};
use bevy::prelude::{Commands, Query, Res, ResMut, Single, Transform, Update, Window};
use crate::game_data::audio_resource::GameAudioResource;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::new(0, 0))
            .add_systems(Update, scoring);
    }
}

fn scoring(
    mut ball_query: Query<(&mut Ball, &mut Transform)>,
    mut score: ResMut<Score>,
    server: Res<AssetServer>,
    audio_resource: Res<GameAudioResource>,
    mut commands: Commands,
    window: Single<&Window>,
) {
    let score_sound = server.load(
        audio_resource
            .search_audio("sounds/score.wav")
            .unwrap_or(SCORE_SOUND.to_string()),
    );

    let blob_sound = server.load(
        audio_resource
            .search_audio("sounds/blob.wav")
            .unwrap_or(BLOB_SOUND.to_string()),
    );

    let play_sound = |cmd: &mut Commands, sound: Handle<AudioSource>, vol: (f32, f32)| {
        cmd.spawn((
            AudioPlayer::new(sound),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(vol.0).decrease_by_percentage(vol.1)),
        ));
    };

    for (mut ball, mut transform) in ball_query.iter_mut() {
        let screen_width = window.width();
        let screen_height = window.height();
        let ball_size = BALL_SIZE * size_factor();
        let ball_x = transform.translation.x;
        let ball_y = transform.translation.y;
        let abs_velocity_x = ball.get_velocity().x.abs();

        if ball_y < ball.get_radius() + FOREGROUND_HEIGHT {
            // add sound
            play_sound(&mut commands, score_sound.clone(), (1.0, 20.0));
            play_sound(&mut commands, blob_sound.clone(), (1.0, 0.0));

            if ball_x <= (screen_width / 2.0) {
                score.set_right(1);
                // Change falling direction
                ball.set_velocity_x(abs_velocity_x);
            } else {
                score.set_left(1);
                ball.set_velocity_x(-abs_velocity_x);
            }

            transform.translation.x = screen_width / 2.0;
            transform.translation.y = screen_height - ball_size;

            ball.set_velocity_y(0.0);
        }
    }
}
