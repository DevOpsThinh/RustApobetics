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

Created At: 21:48 - 10/12/2025
*/
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Color, Component};
use crate::game_objects::character::Side;

#[derive(Debug, Clone, Copy)]
pub struct ScoreLabel {
    pub box_size: Vec2,
    pub box_position: Vec2,
    pub box_background: Color,
    pub text_shadow: Color,
}


#[derive(Component)]
pub struct CharacterHub {
    score_label: ScoreLabel,
    side:Side,
    position: Vec3
}

impl CharacterHub {
    pub fn new(score_label: ScoreLabel, side: Side, position: Vec3) -> Self {
        CharacterHub { score_label, side, position }
    }
    
    pub fn get_score_label(&self) -> &ScoreLabel {
        &self.score_label
    }
    
    pub fn set_side(&mut self, side: Side) {
        self.side = side;
    }
    
    pub fn get_side(&self) -> Side {
        self.side
    }
    
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
    
    pub fn get_position(&self) -> Vec3 {
        self.position
    }
}
