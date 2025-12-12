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

Created At: 14:54 - 04/12/2025
*/
use bevy::prelude::{Component, KeyCode};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn left_direction_key(&self) -> KeyCode {
        match self {
            Side::Left => KeyCode::KeyA,
            Side::Right => KeyCode::Numpad4,
        }
    }

    pub fn right_direction_key(&self) -> KeyCode {
        match self {
            Side::Left => KeyCode::KeyD,
            Side::Right => KeyCode::Numpad6,
        }
    }

    pub fn bounded_range(
        &self, 
        character_width: f32, 
        screen_width: f32
    ) -> (f32, f32) {
        match self {
            Side::Left => (
                character_width / 2.0,
                (screen_width / 2.0) - (character_width / 2.0),
            ),
            Side::Right => (
                (screen_width / 2.0) + (character_width / 2.0),
                screen_width - (character_width / 2.0),
            ),
        }
    }
}

#[derive(Component)]
pub struct Character {
    side: Side,
}

impl Character {
    pub fn new(side: Side) -> Self {
        Character { side }
    }
    // pub fn set_side(&mut self, side: Side) {
    //     self.side = side
    // }
    pub fn get_side(&self) -> &Side {
        &self.side
    }
}
