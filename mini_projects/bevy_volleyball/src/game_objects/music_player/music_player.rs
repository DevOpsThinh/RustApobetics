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

Created At: 02:49 - 11/12/2025
*/
use bevy::prelude::{AudioPlayer, AudioSource, Component, Handle};

pub trait MusicPlayer {
    fn play(&self, source: Handle<AudioSource>) -> AudioPlayer;
}

pub enum AudioType {
    Mp3,
    Wav,
    Aac,
    Flac,
    Ogg,
}

#[derive(Component)]
pub struct GameAudio {
    audio_type: AudioType,
}

impl GameAudio {
    pub fn new(audio_type: AudioType) -> Self {
        GameAudio { audio_type }
    }

    // pub fn set_type(&mut self, audio_type: AudioType) {
    //     self.audio_type = audio_type
    // }
    
    pub fn get_type(&self) -> &AudioType {
        &self.audio_type
    }

    fn mp3_play(&self, source: Handle<AudioSource>) -> AudioPlayer {
        AudioPlayer::new(source)
    }
}

impl MusicPlayer for GameAudio {
    fn play(&self, source: Handle<AudioSource>) -> AudioPlayer {
        match &self.get_type() {
            AudioType::Mp3 => {
                self.mp3_play(source)
            }
            AudioType::Wav => {
                self.mp3_play(source)
            }
            _ => {
                self.mp3_play(source)
            }
            // todo("Handle logic for other music_player types")
            // AudioType::Aac => {}
            // AudioType::Flac => {}
            // AudioType::Ogg => {}
        }
    }
}
