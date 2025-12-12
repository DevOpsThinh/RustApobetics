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

Created At: 21:50 - 10/12/2025
*/
use crate::game_data::score::Score;
use crate::game_objects::character::Side;
use crate::game_objects::score_board::ScoreBoard;
use crate::game_util::constants::{
    BLUE, FONT_PATH, GREEN, LAYER_THREE, RED, SCORE_FONT_SIZE
};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Justify, Res, Single, Text2d, TextFont, TextLayout, Transform, Vec3, Window, default,
};
use bevy::prelude::{Luminance, Query};
use bevy::sprite::Text2dShadow;
use crate::game_ui::hub::{CharacterHub, ScoreLabel};

pub struct HubPlugin;

impl Plugin for HubPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hub_entity)
            .add_systems(Update, update_hub_data);
    }
}

fn update_hub_data(score: Res<Score>, mut query: Query<(&mut Text2d, &ScoreBoard)>) {
    for (mut text, board) in query.iter_mut() {
        text.0 = match board.get_side() {
            Side::Left => score.get_left().to_string(),
            Side::Right => score.get_right().to_string(),
        }
    }
}

fn hub_entity(mut commands: Commands, server: Res<AssetServer>, window: Single<&Window>) {
    let font = server.load(FONT_PATH.to_string());
    let text_font = TextFont {
        font,
        font_size: SCORE_FONT_SIZE,
        ..default()
    };
    let screen_height = window.height();
    let screen_width = window.width();

    let score_label = ScoreLabel {
        box_size: Vec2::new(150.0, 50.0),
        box_position: Vec2::new(0.0, -150.0),
        box_background: Color::srgb(RED, GREEN, BLUE),
        text_shadow: Color::srgb(RED, GREEN, BLUE).darker(0.1),
    };

    let blobs_hub = [
        // Left Align
        CharacterHub::new(
            score_label,
            Side::Left,
            Vec3::new(150.0, screen_height - 50.0, LAYER_THREE),
        ),
        // Right Align
        CharacterHub::new(
            score_label,
            Side::Right,
            Vec3::new(screen_width - 150.0, screen_height - 50.0, LAYER_THREE),
        ),
    ];

    for hub in blobs_hub.iter() {
        let text_align = match hub.get_side() {
            Side::Left => Justify::Left,
            Side::Right => Justify::Right,
        };

        commands.spawn((
            ScoreBoard::new(hub.get_side()),
            Text2d::default(),
            text_font.clone(),
            TextLayout::new_with_justify(text_align),
            Text2dShadow {
                color: hub.get_score_label().text_shadow,
                ..default()
            },
            Transform::from_translation(hub.get_position()),
        ));
    }
}
