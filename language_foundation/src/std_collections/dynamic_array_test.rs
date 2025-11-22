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

Created At: 16:18 - 22/11/2025
*/

#[cfg(test)]
mod dynamic_array_test {
    use crate::std_collections::dynamic_array as dy_arr ;

    #[test]
    fn remove_operation_with_positive_case() {
        let mut _vector = vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "two".to_string(),
        ];

        _vector = dy_arr::remove_element(_vector, String::from("two"));

        assert_eq!(_vector, ["one", "three", "two"]);
    }

    #[test]
    fn add_operation_with_positive_case() {
        let mut _vector = vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string()
        ];

        _vector = dy_arr::add_element(_vector, String::from("four"));
        
        assert_eq!(_vector, ["one", "two", "three", "four"]);
    }
}