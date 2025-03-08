use crate::inventory::Inventory;
use crate::models::Product;
use crate::purchases::Purchases;
use crate::sales::Sales;

pub fn generate_inventory_report(inventory: &Inventory) {
    println!("Inventory Report:");
    for product in inventory.list_products() {
        println!(
            "ID: {}, Name: {}, Quantity: {}, Price: ${:.2}",
            product.id, product.name, product.quantity, product.price
        );
    }
}

pub fn generate_sales_report(sales: &Sales) {
    println!("Sales Report:");
    println!("Total Sales: ${:.2}", sales.total_sales());
}

pub fn generate_purchases_report(purchases: &Purchases) {
    println!("Purchases Report:");
    println!("Total Purchases: ${:.2}", purchases.total_purchases());
}
