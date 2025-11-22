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

Created At: 22:55 - 19/11/2025
*/

pub fn vector_operations() {
    let mut _vector = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string()
    ];
    show_array_value(&_vector);
    _vector = add_element(_vector, String::from("four"));
    show_array_value(&_vector);
    _vector = add_element(_vector, String::from("five"));
    show_array_value(&_vector);
    _vector = remove_element(_vector, String::from("two"));
    show_array_value(&_vector);
    _vector = remove_element(_vector, String::from("one"));

    let _vector = Vec::new();
    let _vector = remove_element(_vector, String::from("one"));
    show_array_value(&_vector);
}

pub fn show_array_value(vector: &Vec<String>) {
    if !valid_vector(vector) {
        println!("\nThe vector must contain at least one value");
    } else {
        println!("{:?}", vector)
    };
}

/// Adds a new element to the end of a vector.
///
/// # Type Parameters
///
/// * `T` - The type of elements contained in the vector.
///
/// # Arguments
///
/// * `vector` - The vector to which the element will be added. Ownership of the vector is taken.
/// * `element` - The element to add to the vector. Ownership of the element is taken.
///
/// # Returns
///
/// Returns the updated vector containing the new element.
///
/// # Example
///
/// ```
/// let v = vec![1, 2];
/// let v = add_element(v, 3);
/// assert_eq!(v, vec![1, 2, 3]);
/// ```
pub fn add_element<T>(mut vector: Vec<T>, element: T) -> Vec<T> {
    vector.push(element); vector
}

/// Removes the first occurrence of a specified element from a vector of strings, if present.
///
/// # Arguments
///
/// * `vector` - The vector of strings from which the element will be removed. Ownership of the vector is taken.
/// * `element` - The string element to remove from the vector. Ownership of the string is taken.
///
/// # Returns
///
/// Returns the updated vector with the first occurrence of `element` removed.
/// If the vector is not valid according to `valid_vector`, the original vector is returned unchanged.
///
/// # Example
///
/// ```
/// let v = vec!["a".to_string(), "b".to_string(), "c".to_string()];
/// let v = remove_element(v, "b".to_string());
/// assert_eq!(v, vec!["a", "c"]);
/// `
pub fn remove_element(mut vector: Vec<String>, element: String) -> Vec<String> {

    if !valid_vector(&vector){
        return vector;
    }

    for i in 0..vector.len() {
        if vector[i] == element {
            vector.remove(i);
            break;
        }
    };
    vector
}

/// Checks whether the provided vector of strings is non-empty.
///
/// # Arguments
///
/// * `vector` - A reference to a vector of strings to be checked.
///
/// # Returns
///
/// Returns `true` if the vector contains at least one element, or `false` if it is empty.
///
/// # Example
///
/// ```
/// let v = vec!["a".to_string()];
/// assert!(valid_vector(&v));
/// let empty: Vec<String> = Vec::new();
/// assert!(!valid_vector(&empty));
/// ``
fn valid_vector<T>(vector: &Vec<T>) -> bool {
    !vector.is_empty()
}

