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

Created At: 01:17 - 28/11/2025
*/

use domain::dto::category::Category;
use domain::dto::inventory::Inventory;
use domain::dto::order::{Order, OrderStatus};
use domain::dto::product::Product;
use domain::dto::user::Customer;
use std::collections::HashMap;
use std::io;

pub fn console_ui() {
    let mut customer = Customer::new("DevOpsThinh", 32);
    let mut to_carts = vec![
        (
            Product::new(
                "Nano Jetson",
                "Electronic",
                109.9,
                vec!["free returns".to_string(), "30 days refund".to_string()],
            ),
            10,
        ),
        (
            Product::new(
                "Soil Measurement Kits",
                "Kit",
                199.9,
                vec!["free returns".to_string(), "30 days refund".to_string()],
            ),
            10,
        ),
        (
            Product::new(
                "ESP32 Programming",
                "Ebook",
                29.9,
                vec!["free returns".to_string(), "30 days refund".to_string()],
            ),
            1,
        ),
    ];
    let mut inventory = make_inventory();
    // Menu
    loop {
        // Menu labels
        make_options_titles();
        // User Input
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input.");
        // Handle user choices (requests)
        let request = user_input.trim();
        handle_user_request(
            Option::from(request),
            &mut customer,
            &mut to_carts,
            &mut inventory,
        );
    }
}

fn handle_user_request(
    request: Option<&str>,
    customer: &mut Customer,
    products: &mut Vec<(Product, u32)>,
    inventory: &mut Inventory,
) {
    let choice = request.unwrap_or("3");

    match choice {
        "1" => {
            add_to_cart(customer, products);

            let cart = customer.get_cart();
            view_products(cart);
        }
        "2" => {
            if !valid_age(customer) {
                println!(
                    "Your age: {} must over 17 to buy our products!",
                    customer.get_age()
                );
                return
            }

            view_products(products);

            println!("Apply the discount to the order: ");
            let discount = get_floating_number_input();
            let order = purchase(customer, discount);
            println!("\nYour order info: {}\n", order);
            let to_export = make_order(customer);
            let export_text = inventory.export_products(to_export);
            show_text_line_by_line(&export_text);
        }
        "3" => {
            let remained_products = inventory.get_products();
            println!("Remained products: {:?}", remained_products);
        }
        "4" => {
            quit_program();
        }
        _ => println!("\nInvalid choice, try again!"),
    }
}

fn show_text_line_by_line(text: &String) {
    let lines = split_text(text, ',');
    for line in lines {
        println!("+ {}", line);
    }
}
/// Splits a given text by a specified delimiter and returns a vector of trimmed parts.
fn split_text(text: &str, delimiter: char) -> Vec<String> {
    // split by delimiter
    text.split(delimiter)
        // trim spaces and convert to String
        .map(|part| part.trim().to_string())
        // remove empty parts
        .filter(|part| !part.is_empty())
        .collect()
}

fn get_floating_number_input() -> f32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read your floating number input ((0.0 ... 1.0]).");

    let f_number = match input.trim().parse::<f32>() {
        Ok(d) if d > 0.0 && d <= 1.0 => d,
        _ => 1.0,
    };

    f_number
}

fn purchase(customer: &mut Customer, discount: f32) -> String {
    let customer_name = customer.get_name();
    let to_order = customer.get_cart();
    let mut order = Order::new(
        "23:59:59-1/1/2026-DevOpsThinh-123",
        customer_name,
        [to_order],
        "23:59:59-1/1/2026",
    );

    order.update_status(OrderStatus::Packed);

    let mut total = 0.0;

    for (mut p, q) in to_order.clone() {
        p.apply_discount(discount);

        let price = p.get_price() * (q as f64);
        total += price;
    }
    // 1.5 => 2.0, 1.4 => 1.0
    let total = total.round();

    let mut info = String::new();

    info.push_str(order.get_order_id());
    info.push_str(", ");
    info.push_str(order.get_customer());
    info.push_str(", ");
    info.push_str(order.get_date());
    info.push_str(", ");
    info.push_str(order.get_status());
    info.push_str(", ");
    info.push_str(total.to_string().as_str());
    info.push_str("$");

    info
}

fn make_order(customer: &mut Customer) -> Vec<(Product, u32)> {
    let _customer = customer.clone();
    let to_order = _customer.get_cart();

    to_order.clone()
}

fn make_inventory() -> Inventory {
    let cats = add_categories_inventory();

    let vendor_products: Vec<(Product, u32)> = vec![
        (
            Product::new(
                "Nano Jetson",
                cats[0].get_name(),
                109.9,
                vec!["free returns".to_string(), "30 days refund".to_string()],
            ),
            100,
        ),
        (
            Product::new(
                "Soil Measurement Kits",
                cats[1].get_name(),
                199.9,
                vec!["free returns".to_string(), "30 days refund".to_string()],
            ),
            100,
        ),
        (
            Product::new(
                "ESP32 Programming",
                cats[2].get_name(),
                29.9,
                vec!["free returns".to_string(), "30 days refund".to_string()],
            ),
            u32::MAX,
        ),
    ];

    let products = add_products_inventory(vendor_products);

    let mut inventory = Inventory::new("Vietnam 1");
    inventory.add_categories(cats);
    inventory.import_products(products);

    inventory
}

fn add_products_inventory(products: Vec<(Product, u32)>) -> HashMap<Product, u32> {
    let mut hm_products: HashMap<Product, u32> = HashMap::new();

    for (p, q) in products {
        hm_products
            .entry(p)
            .and_modify(|e| *e += q)
            .or_insert(q);
    }

    hm_products
}

fn add_categories_inventory() -> Vec<Category> {
    let mut categories: Vec<Category> = Vec::new();
    categories.push(Category::new("Electronic"));
    categories.push(Category::new("Kit"));
    categories.push(Category::new("Ebook"));
    categories
}

fn view_products(products: &Vec<(Product, u32)>) {
    let mut index: u8 = 1;
    for (p, q) in products {
        println!("{}. {} -> {}\r", index, p, q);
        index += 1;
    }
}

fn add_to_cart(customer: &mut Customer, products: &Vec<(Product, u32)>) {
    for p in products {
        customer.add_to_cart(p.clone());
    }
}

fn valid_age(customer: &Customer) -> bool {
    customer.get_age() > &17
}

fn quit_program() {
    println!("Quitting...\nThank you for playing Rust!");
    std::process::exit(0);
}

fn make_options_titles() {
    println!("\n***Elektor Store***\n\rPlease choose an option:");
    println!(
        r#"
        1. Add products to cart
        2. Make purchase
        3. View inventory
        4. Quit "#
    );
}
