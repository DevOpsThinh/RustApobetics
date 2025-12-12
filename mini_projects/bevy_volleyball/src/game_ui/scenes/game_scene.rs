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

Created At: 00:56 - 04/12/2025
*/
use crate::game_data::audio_resource::GameAudioResource;
use crate::game_objects::music_player::{AudioType, GameAudio, MusicPlayer};
use crate::game_ui::scene::Scene;
use crate::game_util::constants::{
    BACKGROUND_PATH, FOREGROUND_HEIGHT, FOREGROUND_PATH, 
    FOREGROUND_Y, LAYER_MAX, LAYER_ONE, MUSIC_BACKGROUND, 
    PHYSIC_HEIGHT, PHYSIC_WIDTH,
};
use crate::game_util::texture_manipulation::{SpriteType, load_texture};
use bevy::audio::{PlaybackSettings, Volume};
use bevy::prelude::{
    AssetServer, Commands, Res, Single, SpawnRelated, Sprite, SpriteImageMode, 
    Transform, UVec2, Vec2, children, default,
};
use bevy::{
    camera::{Camera, Camera2d, Viewport},
    window::Window,
};

// const POSITION_SCALE: f32 = 0.125;
// const SIZE_SCALE: f32 = 0.75;
const POSITION_SCALE: f32 = 0.0;
const SIZE_SCALE: f32 = 1.0;

pub fn setup_2dworld(
    mut commands: Commands,
    mut server: Res<AssetServer>,
    music_resource: Res<GameAudioResource>,
    window: Single<&Window>,
) {
    let window_size = window.resolution.physical_size().as_vec2();

    camera_entity(
        &mut commands,
        (window_size * POSITION_SCALE).as_uvec2(),
        (window_size * SIZE_SCALE).as_uvec2(),
    );
    scenes_entity(&mut commands, &mut server, music_resource, window);
}

fn camera_entity(commands: &mut Commands, pos: UVec2, size: UVec2) {
    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: pos,
                physical_size: size,
                ..default()
            }),
            ..default()
        },
        Transform::from_xyz(
            PHYSIC_WIDTH as f32 / 2.0,
            PHYSIC_HEIGHT as f32 / 2.0,
            LAYER_MAX,
        ),
    ));
}

fn scenes_entity(
    commands: &mut Commands,
    server: &mut Res<AssetServer>,
    music_resource: Res<GameAudioResource>,
    window: Single<&Window>,
) {
    let background = load_texture(BACKGROUND_PATH, server, SpriteType::ImageSet);
    let foreground = load_texture(FOREGROUND_PATH, server, SpriteType::ImageSet);
    let width = window.resolution.physical_width() as f32;
    let height = window.resolution.physical_height() as f32;

    let music = music_resource
        .search_audio("music/music_background.mp3")
        .unwrap_or(MUSIC_BACKGROUND.to_string());
    let source = server.load(music);

    let scene = Scene {
        size: Vec2::new(width, height),
        background,
        foreground,
        scale_mode: SpriteImageMode::Auto,
        music: source,
    };

    commands.spawn((
        GameAudio::new(AudioType::Mp3).play(scene.music),
        PlaybackSettings::LOOP
            .with_volume(Volume::decrease_by_percentage(&Volume::Linear(1.0), 75.0)),
        Sprite {
            image: scene.background,
            custom_size: Some(scene.size),
            image_mode: scene.scale_mode.clone(),
            ..default()
        },
        children![(
            Sprite {
                image: scene.foreground,
                custom_size: Some(Vec2::new(width, FOREGROUND_HEIGHT)),
                image_mode: scene.scale_mode,
                ..default()
            },
            Transform::from_xyz(0.0, FOREGROUND_Y, LAYER_ONE),
        ),],
        Transform::from_xyz(width / 2.0, height / 2.0, 0.0),
    ));
}
