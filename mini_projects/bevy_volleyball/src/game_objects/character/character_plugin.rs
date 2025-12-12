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

Created At: 15:37 - 05/12/2025
*/
use crate::game_objects::character::{Character, Side};
use crate::game_util::constants::{
    BLOB_HEIGHT, BLOB_PATH, BLOB_SPEED, BLOB_WIDTH,
    FOREGROUND_HEIGHT, LAYER_ONE
};
use crate::game_util::space_factors::{size_factor};
use bevy::app::{App, Startup};
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{
    ButtonInput, Commands, Handle, Image, KeyCode, Plugin, Query, Res,
    Sprite, SpriteImageMode, Time, Transform, Update, default, Vec3,
    Window, Single
};

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, blobs_entity)
            .add_systems(Update, blobs_control);
    }
}

fn blobs_control(
    k_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Character, &mut Transform)>,
    window: Single<&Window>,
) {
    let screen_width = window.width();
    let blob_width = BLOB_WIDTH * size_factor();
    for (blob, mut transform) in query.iter_mut() {
        let left = if k_input.pressed(blob.get_side().left_direction_key()) {
            -1.0
        } else { 0.0 };

        let right = if k_input.pressed(blob.get_side().right_direction_key()) {
            1.0
        } else { 0.0 };

        let hor_direction = left + right;
        let offset = hor_direction * BLOB_SPEED * time.delta_secs();
        // Apply movement deltas for each blob character
        transform.translation.x += offset;
        // Get and apply bounded ranges for each blob character in the game  screen
        let (left_bounded, right_bounded) = blob.get_side().bounded_range(blob_width, screen_width);
        transform.translation.x = transform.translation.x.clamp(left_bounded, right_bounded)
    }
}
/// Blob characters
fn blobs_entity(
    mut commands: Commands,
    server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let screen_width = window.width();
    let blob_width = BLOB_WIDTH * size_factor();
    let blob_height = BLOB_HEIGHT * size_factor();
    let blob_image: Handle<Image> = server.load(BLOB_PATH.to_string());
    let characters = [
        (
            // side, image, size, scale mode, flip_x, xyz_positions
            Side::Left,
            blob_image.clone(),
            Vec2::new(blob_width, blob_height),
            SpriteImageMode::Auto,
            false,
            Vec3::new(blob_width / 2.0, blob_height / 2.0 + FOREGROUND_HEIGHT, LAYER_ONE),
        ),
        (
            // side, image, size, scale mode, flip_x, xyz_positions
            Side::Right,
            blob_image,
            Vec2::new(blob_width, blob_height),
            SpriteImageMode::Auto,
            true,
            Vec3::new(screen_width - (blob_width / 2.0), blob_height / 2.0 + FOREGROUND_HEIGHT, LAYER_ONE),
        ),
    ];

    for (side, image, size, scale, flip_x, pos) in characters {
        commands.spawn((
            Character::new(side),
            Sprite {
                image,
                custom_size: Some(size),
                image_mode: scale,
                flip_x,
                ..default()
            },
            Transform::from_translation(Vec3::new(pos.x, pos.y, pos.z)),
        ));
    }
}
