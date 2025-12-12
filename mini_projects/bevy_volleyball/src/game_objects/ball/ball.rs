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

Created At: 15:07 - 06/12/2025
*/
use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Ball {
    radius: f32,
    velocity: Vec2
}

impl Ball {
    pub fn new(radius: f32, velocity: Vec2) -> Self {
        Ball { radius, velocity }
    }
    
    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    // pub fn set_velocity(&mut self, velocity: Vec2) {
    //     self.velocity = velocity;
    // }

    pub fn get_velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn update_velocity_y(&mut self, delta_time: f32) {
        self.velocity.y += delta_time;
    }

    pub fn set_left_velocity_x(&mut self, x_velocity: f32) {
        self.velocity.x = x_velocity;
    }

    pub fn set_right_velocity_x(&mut self, x_velocity: f32) {
        self.velocity.x = -x_velocity;
    }
    
    pub fn set_velocity_x(&mut self, x_pos: f32) {
        self.velocity.x = x_pos
    }
    
    pub fn set_velocity_y(&mut self, y_pos: f32) {
        self.velocity.y = y_pos
    }
}