mod auth;
mod inventory;
mod models;
mod purchases;
mod reports;
mod sales;

use crate::auth::Auth;
use crate::inventory::Inventory;
use crate::models::Product;
use crate::purchases::Purchases;
use crate::reports::{generate_inventory_report, generate_purchases_report, generate_sales_report};
use crate::sales::Sales;
use std::io::{self, Write};

// Function to read a floating-point number from user input
fn read_amount(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(amount) if amount > 0.0 => return amount,
            _ => println!("Invalid input. Please enter a positive number."),
        }
    }
}

// Function to read a string from user input
fn read_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Function to read an unsigned integer from user input
fn read_u32(prompt: &str) -> u32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<u32>() {
            Ok(value) => return value,
            _ => println!("Invalid input. Please enter a positive integer."),
        }
    }
}


fn main() {
    let mut inventory = Inventory::new();
    let mut sales = Sales::new();
    let mut purchases = Purchases::new();
    let auth = Auth::new("admin".to_string(), "password".to_string());

    println!("Welcome to Rust Store Inventory Management System!");

    //Authenticate user
    let mut username = String::new();
    let mut password = String::new();

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    print!("Enter password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();

    if !auth.authenticate(username.trim(), password.trim()) {
        println!("Authentication failed. Exiting...");
        return;
    }

    loop {
        println!("\nMenu:");
        println!("1. Add Product");
        println!("2. Record Sale");
        println!("3. Record Purchase");
        println!("4. Generate Reports");
        println!("5. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                // Add product logic
                let id = read_u32("Enter product ID: ");
                let name = read_string("Enter product name: ");
                let description = read_string("Enter product description: ");
                let price = read_amount("Enter product price: ");
                let quantity = read_u32("Enter product quantity: ");

                let product = Product {
                    id,
                    name,
                    description,
                    price,
                    quantity,
                };

                match inventory.add_product(product) {
                    Ok(()) => println!("Product added successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "2" => {
                // Record sale logic
                let product_id = read_u32("Enter product ID: ");
                let quantity_sold = read_u32("Enter quantity sold: ");
                let sale_price = read_amount("Enter sale price: ");

                match sales.record_sale(&mut inventory, product_id, quantity_sold, sale_price) {
                    Ok(()) => println!("Sale recorded successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "3" => {
                // Record purchase logic
                let product_id = read_u32("Enter product ID: ");
                let quantity_purchased = read_u32("Enter quantity purchased: ");
                let purchase_price = read_amount("Enter purchase price: ");

                match purchases.record_purchase(&mut inventory, product_id, quantity_purchased, purchase_price) {
                    Ok(()) => println!("Purchase recorded successfully!"),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "4" => {
                generate_inventory_report(&inventory);
                generate_sales_report(&sales);
                generate_purchases_report(&purchases);
            }
            "5" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
