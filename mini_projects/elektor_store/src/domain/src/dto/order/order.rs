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

Created At: 00:03 - 29/11/2025
*/

pub enum OrderStatus {
    Pending,
    Packed,
    Shipping,
    Delivered,
    Reviewed,
}

pub struct Order<C, P, const N: usize> {
    order_id: &'static str,
    customer: C,
    products: [P; N],
    date: String,
    status: OrderStatus,
}

impl<C, P, const N: usize> Order<C, P, N> {
    pub fn new(id: &'static str, customer: C, products: [P; N], date: &str) -> Self {
        Order {
            order_id: id,
            customer,
            products,
            date: date.to_string(),
            status: OrderStatus::Pending,
        }
    }

    pub fn get_order_id(&self) -> &'static str {
        &self.order_id
    }

    pub fn get_customer(&self) -> &C {
        &self.customer
    }

    pub fn get_products(&self) -> &[P; N] {
        &self.products
    }

    pub fn get_date(&self) -> &String {
        &self.date
    }

    pub fn get_status(&self) -> &str {
        match &self.status {
            OrderStatus::Pending => "Pending",
            OrderStatus::Packed => "Packed",
            OrderStatus::Shipping => "Shipping",
            OrderStatus::Delivered => "Delivered",
            OrderStatus::Reviewed => "Completed",
        }
    }

    pub fn update_products(&mut self, products: [P; N]) {
        self.products = products
    }

    pub fn update_status(&mut self, status: OrderStatus) {
        self.status = status
    }
}
