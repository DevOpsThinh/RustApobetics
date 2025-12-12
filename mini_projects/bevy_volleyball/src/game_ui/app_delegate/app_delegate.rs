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

Created At: 00:18 - 04/12/2025
*/
use crate::game_data::audio_plugin::GameAudioPlugin;
use crate::game_data::score_plugin::ScorePlugin;
use crate::game_objects::ball_plugin::BallPlugin;
use crate::game_objects::character_plugin::CharactersPlugin;
use crate::game_ui::game_scene::setup_2dworld;
use crate::game_ui::hub_plugin::HubPlugin;
use crate::game_util::constants::{
    APP_NAME, ASSETS_PATH, PHYSIC_HEIGHT, PHYSIC_WIDTH
};
use bevy::DefaultPlugins;
use bevy::ecs::system::NonSendMarker;
use bevy::prelude::{BevyError, Entity, Single, With};
use bevy::window::PrimaryWindow;
use bevy::winit::WINIT_WINDOWS;
use bevy::{
    prelude::{
        App, AssetPlugin, MonitorSelection, PluginGroup, Startup, Window,
        WindowPlugin, WindowPosition, default,
    },
    window::WindowResolution,
};
use std::io::Cursor;
use winit::window::Icon;

pub fn app_delegate() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(setup_window())
                .set(setup_assets(ASSETS_PATH)),
        )
        .add_plugins((
            CharactersPlugin,
            BallPlugin,
            ScorePlugin,
            HubPlugin,
            GameAudioPlugin,
        ))
        .add_systems(Startup, (setup_window_icon, setup_2dworld))
        .run();
}

fn setup_window_icon(
    primary_window: Single<Entity, With<PrimaryWindow>>,
    _non_send_marker: NonSendMarker,
) -> Result<(), BevyError> {
    WINIT_WINDOWS.with_borrow(|windows| {
        let Some(primary) = windows.get_window(*primary_window) else {
            return Err(BevyError::from("No primary window!"));
        };

        let icon_buf = Cursor::new(include_bytes!("../../../icon.png"));

        if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
            let image = image.into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            let icon = Icon::from_rgba(rgba, width, height).unwrap();
            primary.set_window_icon(Some(icon));
        };

        Ok(())
    })
}

fn setup_assets(path: &str) -> AssetPlugin {
    AssetPlugin {
        file_path: path.into(),
        ..default()
    }
}

fn setup_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: APP_NAME.into(),
            resolution: WindowResolution::new(PHYSIC_WIDTH, PHYSIC_HEIGHT),
            resizable: false,
            position: WindowPosition::Centered(MonitorSelection::Primary),
            ..default()
        }),
        ..default()
    }
}
