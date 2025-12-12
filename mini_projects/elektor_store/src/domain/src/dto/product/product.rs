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

Created At: 23:57 - 27/11/2025
*/
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum ProductStatus {
    InStock,
    OutOfStock,
    CheckedOut(String),
}

impl Display for ProductStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InStock => "InStock",
                Self::OutOfStock => "OutOfStock",
                Self::CheckedOut(text) => text,
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Product {
    name: String,
    category: String,
    price: f64,
    tags: Vec<String>,
    status: ProductStatus,
}

impl Product {
    pub fn new(name: &str, category: &str, price: f64, tags: Vec<String>) -> Self {
        Product {
            name: name.to_string(),
            category: category.to_string(),
            price,
            tags,
            status: ProductStatus::InStock,
        }
    }

    pub fn apply_discount(&mut self, discount: f32) {
        self.price = self.price * discount as f64;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_category(&self) -> &String {
        &self.category
    }

    pub fn set_price(mut self, price: f64){
        self.price = price;
    }

    pub fn get_price(&self) -> &f64 {
        &self.price
    }

    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    pub fn set_status(&mut self, status: ProductStatus) {
        self.status = status;
    }

    pub fn get_status(&self) -> &str {
        match &self.status {
            ProductStatus::InStock => "InStock",
            ProductStatus::OutOfStock => "OutOfStock",
            ProductStatus::CheckedOut(info) => info,
        }
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, ({}) - Status: { }",
            self.name,
            self.category,
            self.price,
            self.tags.join(", "),
            self.status
        )
    }
}

impl Hash for Product {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.category.hash(state);
    }
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.category == other.category
    }
}

impl Eq for Product {}
