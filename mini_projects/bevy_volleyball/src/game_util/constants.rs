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

Created At: 14:26 - 05/12/2025
*/
// ============================ RESOLUTIONS TESTING
// qHD
pub const PHYSIC_WIDTH: u32 = 960;
pub const PHYSIC_HEIGHT: u32 = 540;
// WXGA HD
// pub const PHYSIC_WIDTH: u32 = 1366;
// pub const PHYSIC_HEIGHT: u32 = 768;
// WXGA
// pub const PHYSIC_WIDTH: u32 = 1280;
// pub const PHYSIC_HEIGHT: u32 = 800;
// 1080p - Full HD
// pub const PHYSIC_WIDTH: u32 = 1920;
// pub const PHYSIC_HEIGHT: u32 = 1080;
// 1440p - QHD (2K)
// pub const PHYSIC_WIDTH: u32 = 2560;
// pub const PHYSIC_HEIGHT: u32 = 1440;

// ============================ SOUNDS & MUSIC
pub const MUSIC_BACKGROUND: &str = "music/music_background.mp3";
pub const BLOB_SOUND: &str = "sounds/blob.wav";
pub const BALL_BOUNCE_SOUND: &str = "sounds/ball_bounce.wav";
pub const SCORE_SOUND: &str = "sounds/score.wav";

// ============================ HUB
pub const FONT_PATH: &str = "fonts/Nosifer-Regular.ttf";
pub const SCORE_FONT_SIZE: f32 = 40.0;

// ============================ CHARACTER
pub const BLOB_PATH: &str = "sprites/blobs/blob_walk_1/blob-walk_0@1x.png";
pub const BLOB_HEIGHT: f32 = 98.0;
pub const BLOB_WIDTH: f32 = 99.0;
pub const BLOB_SPEED: f32 = 50.0;

// ============================ BALL
pub const BALL_SHEET_PATH: &str = "textures/ball_1/ball_sheet_1@1x.png";
pub const BALL_SIZE: f32 = 89.0;
pub const BALL_FRAMES: u32 = 8;
pub const BALL_X_VELOCITY: f32 = 40.0;
pub const BALL_Y_VELOCITY: f32 = 0.0;
pub const BALL_RADIUS: f32 = 44.5;
/// Gravity Acceleration
pub const BALL_GRAVITY_AC: f32 = -40.0;
pub const G_FACTOR: f32 = 2.0;
pub const RANGE: (f32, f32) = (0.8, 1.8);

// ============================ POSITIONS & RATIOS
pub const Y_RATIO: f32 = 5.0;
pub const FOREGROUND_HEIGHT: f32 = (PHYSIC_HEIGHT as f32) / Y_RATIO;
pub const FOREGROUND_Y: f32 = (-(PHYSIC_HEIGHT as f32) / 2.0) + ((PHYSIC_HEIGHT as f32)/ Y_RATIO / 2.0);

// ============================ LAYER PRIORITY
pub const LAYER_ONE: f32 = 1.0;
pub const LAYER_TWO: f32 = 2.0;
pub const LAYER_THREE: f32 = 3.0;
pub const LAYER_MAX: f32 = 99.0;

// ============================ COLORS
pub const RED: f32 = 105.0 / 255.0;
pub const GREEN: f32 = 157.0 / 255.0;
pub const BLUE: f32 = 181.0 / 255.0;

// ============================ APP RESOURCES
pub const ASSETS_PATH: &str = "src/assets";
pub const APP_NAME: &str = "Bevy Volleyball";
pub const BACKGROUND_PATH: &str = "textures/background_1/background_1@1x.png";
pub const FOREGROUND_PATH: &str = "textures/foreground_1/foreground_1@1x.png";
