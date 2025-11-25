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

Created At: 17:38 - 25/11/2025
*/

/// A classical linear search algorithm for an integer 32  array with five elements
pub fn search(k: i32, my_arr: [i32; 5] ) -> bool {
    for i in my_arr.iter() {
        if k == *i {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_negative_case() {
        let result = search(6, [1, 2, 3, 4, 5]);
        assert_eq!(result, false);
    }

    #[test]
    fn it_works_with_positive_case() {
        let result = search(3, [1, 2, 3, 4, 5]);
        assert_eq!(result, true);
    }
}
