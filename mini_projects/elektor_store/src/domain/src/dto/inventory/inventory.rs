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

Created At: 21:43 - 29/11/2025
*/
use crate::dto::category::Category;
use crate::dto::product::{Product};
use std::collections::HashMap;

pub struct Inventory {
    name: String,
    products: HashMap<Product, u32>,
    categories: Vec<Category>,
}

impl Inventory {
    pub fn new(name: &str) -> Self {
        Inventory {
            name: name.to_string(),
            products: HashMap::new(),
            categories: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn import_products(&mut self, products: HashMap<Product, u32>) {
        self.products = products
    }

    pub fn get_products(&self) -> &HashMap<Product, u32> {
        &self.products
    }

    pub fn add_category(&mut self, category: Category) {
        self.categories.push(category);
    }

    pub fn add_categories(&mut self, categories: Vec<Category>) {
        self.categories = categories;
    }

    pub fn get_categories(&self) -> &Vec<Category> {
        &self.categories
    }

    pub fn export_products(&mut self, export_list: Vec<(Product, u32)>) -> String {
        let mut message = String::new();

        for (p, q) in export_list {
            match self.products.get_mut(&p) {
                Some(e) => {
                    if *e >= q {
                        *e -= q;
                        message.push_str("Customer bought: " );
                        message.push_str(q.to_string().as_str());
                        message.push_str(" ");
                        message.push_str(p.get_name().as_str());
                        message.push_str(" ");
                        message.push_str(p.get_price().to_string().as_str());
                        message.push('$');
                        message.push(',')
                    } else {
                        message.push_str("Insufficient stock for: ");
                        message.push_str(&p.get_name());
                        message.push(',');
                    }
                }
                None => {
                    message.push_str("Item: ");
                    message.push_str(&p.get_name());
                    message.push_str(" out of stock!");
                    message.push(',');
                }
            }
        }

        message
    }
}

