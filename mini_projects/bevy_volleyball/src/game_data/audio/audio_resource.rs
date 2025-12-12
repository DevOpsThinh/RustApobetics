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

Created At: 16:09 - 11/12/2025
*/
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameAudioResource {
    items: Vec<String>,
}

impl GameAudioResource {
    pub fn new(list: Vec<&str>) -> Self {
        GameAudioResource {
            items: list.iter().map(|s| s.to_string()).collect(),
        }
    }
    
    // pub fn add_audio(&mut self, item: &str) {
    //     self.items.push(item.to_string());
    // }
    
    // pub fn remove_audio(&mut self, item: &str) {
    //     self.items.retain(|item| item != item);
    // }
    
    pub fn search_audio(&self, audio_path: &'static str) -> Option<String> {
        let mut item = "";
        for audio in &self.items {
            if audio.eq(audio_path) {
                item = audio;
                break;
            }
        }
      
        Some(item.to_string())
    }

    // pub fn audio_list(&self) -> &Vec<String> {
    //     &self.items
    // }
}