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

Created At: 14:20 - 25/11/2025
*/

/// The `Cipher` trait, which will construct the cryptography encrypt/ decrypt methods.
///
/// # Type Parameters
///
/// * `T` - The given `T` type.
pub trait Cipher<T> {
    fn encrypt(&self, data_entry: T) -> String;
    fn decrypt(&self, data_entry: T) -> String;
}

const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz-123456789#%";
const DEFAULT_CHAR: &'static char = &'%';
const DEFAULT_PAD_DIGIT: &'static char = &'1';

///The `VigenereCipher` struct represents the Vigenère Cipher .
///
/// Cryptography algorithm, which is a method of encrypting alphabetic text.
///
/// The program algorithm code is inspired by Prof Hamzeh Roumani PhD (Department of Electrical Engineering and Computer Science York University, Toronto, Ontario, Canada).
///
/// For more information, visit [Vigenère Cipher](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher)
pub struct VigenereCipher {
    key: &'static str,
}

impl VigenereCipher {
    pub fn new(key: &'static str) -> VigenereCipher {
        VigenereCipher { key }
    }

    fn decrypt_algorithm(&self, data_entry: &str, pad: &str) -> String {
        let mut decrypted = String::new();

        for (index, char) in data_entry.chars().enumerate() {
            let i = ALPHABET
                .chars()
                .into_iter()
                .position(|c| c == char)
                .unwrap_or(ALPHABET.len() - 1);
            let (j, pc) = pad
                .char_indices()
                .nth(index)
                .unwrap_or((0, pad.chars().nth(0).unwrap_or(*DEFAULT_PAD_DIGIT)));

            let mut new_index = i - j;
            while &new_index < &0 {
                new_index += ALPHABET.len();
            }
            new_index %= ALPHABET.len();

            let chr = ALPHABET
                .chars()
                .into_iter()
                .nth(new_index)
                .unwrap_or(*DEFAULT_CHAR);
            decrypted.push_str(chr.to_string().as_str());
        }

        decrypted
    }

    fn encrypt_algorithm(&self, data_entry: &str, pad: &str) -> String {
        let mut encrypted = String::new();

        for (index, char) in data_entry.chars().enumerate() {
            let i = ALPHABET
                .chars()
                .into_iter()
                .position(|c| c == char)
                .unwrap_or(ALPHABET.len() - 1);
            let (j, pc) = pad
                .char_indices()
                .nth(index)
                .unwrap_or((0, pad.chars().nth(0).unwrap_or(*DEFAULT_PAD_DIGIT)));

            let new_index = (i + j) % ALPHABET.len();

            let chr = ALPHABET
                .chars()
                .into_iter()
                .nth(new_index)
                .unwrap_or(*DEFAULT_CHAR);
            encrypted.push_str(chr.to_string().as_str());
        }

        encrypted
    }

    fn initialize_pad(&self, entry: &str) -> String {
        let mut pad = self.key.to_string();
        while pad.len() < entry.len() {
            pad.push_str(self.key);
        }
        pad
    }
}

impl Cipher<&str> for VigenereCipher {
    fn encrypt(&self, data_entry: &str) -> String {
        self.encrypt_algorithm(
            data_entry,
            self.initialize_pad(data_entry).as_str()
        )
    }

    fn decrypt(&self, data_entry: &str) -> String {
        self.decrypt_algorithm(
            data_entry,
            self.initialize_pad(data_entry).as_str()
        )
    }
}
