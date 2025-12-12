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

Created At: 15:17 - 06/12/2025
*/
use crate::game_data::audio_resource::GameAudioResource;
use crate::game_objects::ball::Ball;
use crate::game_objects::character::{Character, Side};
use crate::game_util::constants::{
    BALL_BOUNCE_SOUND, BALL_FRAMES, BALL_GRAVITY_AC, BALL_RADIUS, BALL_SHEET_PATH, BALL_SIZE,
    BALL_X_VELOCITY, BALL_Y_VELOCITY, BLOB_HEIGHT, BLOB_WIDTH, FOREGROUND_HEIGHT, G_FACTOR,
    LAYER_ONE, RANGE,
};
use crate::game_util::space_factors::size_factor;
use crate::game_util::texture_manipulation::{atlas_layout, index_texture_for};
use bevy::app::App;
use bevy::asset::{AssetServer, Handle};
use bevy::audio::{PlaybackSettings, Volume};
use bevy::math::Vec2;
use bevy::prelude::{Assets, AudioPlayer, Commands, Plugin, Query, Res, ResMut, Single, Sprite, Startup, TextureAtlas, TextureAtlasLayout, Transform, Update, Window};
use bevy::time::Time;
use bevy::utils::default;
use rand::Rng;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, ball_entity)
            .add_systems(Update, (ball_motion, bounce_effect));
    }
}

fn bounded_collision(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}

fn bounce_effect(
    mut ball_query: Query<(&mut Ball, &Transform)>,
    blob_query: Query<(&Character, &Transform)>,
    server: Res<AssetServer>,
    audio_resource: Res<GameAudioResource>,
    mut commands: Commands,
    window: Single<&Window>,
) {
    let amount = rand::rng().random_range(RANGE.0..RANGE.1);
    let screen_width = window.width();
    let screen_height = window.height();
    let blob_width = BLOB_WIDTH * size_factor();
    let blob_height = BLOB_HEIGHT * size_factor();

    let bounce_sound = server.load(
        audio_resource
            .search_audio("sounds/ball_bounce.wav")
            .unwrap_or(BALL_BOUNCE_SOUND.to_string()),
    );

    let mut play_bounce_sound = || {
        commands.spawn((
            AudioPlayer::new(bounce_sound.clone()),
            PlaybackSettings::ONCE.with_volume(Volume::Linear(1.0).decrease_by_percentage(10.0)),
        ));
    };

    for (mut ball, ball_transform) in ball_query.iter_mut() {
        let ball_x = ball_transform.translation.x;
        let ball_y = ball_transform.translation.y;
        let velocity_x = ball.get_velocity().x;
        let velocity_y = ball.get_velocity().y;
        let ball_radius = ball.get_radius();

        // Bottom -> Up
        if ball_y <= ball_radius + FOREGROUND_HEIGHT && ball.get_velocity().y < 0.0 {
            ball.set_velocity_y(-velocity_y);
        } else if ball_y >= screen_height - ball_radius && ball.get_velocity().y > 0.0 {
            // add sound
            play_bounce_sound();
            ball.set_velocity_y(-velocity_y);
            // Right -> left
        } else if ball_x <= ball_radius && ball.get_velocity().x < 0.0 {
            // add sound
            play_bounce_sound();
            ball.set_velocity_x(-velocity_x);
        } else if ball_x >= (screen_width - ball_radius) && ball.get_velocity().x > 0.0 {
            // add sound
            play_bounce_sound();
            ball.set_velocity_x(-velocity_x);
        }

        for (blob, blob_transform) in blob_query.iter() {
            let blob_x = blob_transform.translation.x;
            let blob_y = blob_transform.translation.y;

            if bounded_collision(
                ball_x,
                ball_y,
                blob_x - blob_width / 2.0 - ball_radius,
                blob_y - blob_height / 2.0 - ball_radius,
                blob_x + blob_width / 2.0 + ball_radius,
                blob_y + blob_height / 2.0 + ball_radius,
            ) {
                // Bounce effect & ball move directions when ball contact with a blob character
                if ball.get_velocity().y < 0.0 {
                    // add sound
                    play_bounce_sound();
                    // Only bounce when ball is falling
                    ball.set_velocity_y(-velocity_y);

                    match blob.get_side() {
                        Side::Left => ball.set_left_velocity_x(velocity_x.abs() * amount),
                        Side::Right => ball.set_right_velocity_x(velocity_x.abs() * amount),
                    }
                }
            }
        }
    }
}

fn ball_motion(time: Res<Time>, mut query: Query<(&mut Ball, &mut Transform)>) {
    for (mut b, mut transform) in query.iter_mut() {
        // Apply movement deltas for ball
        transform.translation.x += b.get_velocity().x * time.delta_secs(); // Comment it to keep the ball moving straight down
        transform.translation.y += (b.get_velocity().y
            + time.delta_secs() * BALL_GRAVITY_AC / G_FACTOR)
            * time.delta_secs();

        b.update_velocity_y(time.delta_secs() * BALL_GRAVITY_AC);
    }
}

fn ball_entity(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    window: Single<&Window>,
) {
    let screen_width = window.width();
    let screen_height = window.height();
    let ball_size = BALL_SIZE * size_factor();
    let ball_radius = BALL_RADIUS * size_factor();
    let ball_image = server.load(BALL_SHEET_PATH.to_string());
    let atlas_layout: Handle<TextureAtlasLayout> = atlas_layout(
        &mut atlas_layouts,
        ball_size as u32,
        (BALL_FRAMES, 1),
        (None, None),
    );

    commands.spawn((
        Ball::new(ball_radius, Vec2::new(BALL_X_VELOCITY, BALL_Y_VELOCITY)),
        Sprite {
            image: ball_image,
            texture_atlas: Some(TextureAtlas {
                layout: atlas_layout,
                index: index_texture_for(0, BALL_FRAMES as usize),
            }),
            ..default()
        },
        Transform::from_xyz(screen_width / 2.0, screen_height - ball_size, LAYER_ONE),
    ));
}
