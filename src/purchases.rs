use crate::inventory::Inventory;
use crate::models::{Product, Purchase};

pub struct Purchases {
    purchase_history: Vec<Purchase>,
}

impl Purchases {
    pub fn new() -> Self {
        Purchases {
            purchase_history: Vec::new(),
        }
    }
    pub fn record_purchase(
        &mut self,
        inventory: &mut Inventory,
        product_id: u32,
        quantity_purchased: u32,
        purchase_price: f64,
    ) -> Result<(), String> {
        if let Some(product) = inventory.get_product(product_id) {
            let purchase = Purchase{
                product_id,
                quantity_purchased,
                purchase_price,
            };
            self.purchase_history.push(purchase);
            Ok(())
        } else {
            Err("Product not found".to_string())
        }
    }
    pub fn total_purchases(&self) -> f64 {
        self.purchase_history
            .iter()
            .map(|purchase| purchase.quantity_purchased as f64 * purchase.purchase_price)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_purchase() {
        let mut inventory = Inventory::new();
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 100,
        };
        inventory.add_product(product).unwrap();

        let mut purchases = Purchases::new();

        // Record a purchase
        assert!(purchases
            .record_purchase(&mut inventory, 1, 10, 5.0)
            .is_ok());

        // Verify the purchase was recorded
        assert_eq!(purchases.purchase_history.len(), 1);
        assert_eq!(purchases.purchase_history[0].product_id, 1);
        assert_eq!(purchases.purchase_history[0].quantity_purchased, 10);
        assert_eq!(purchases.purchase_history[0].purchase_price, 5.0);
    }

    #[test]
    fn test_record_purchase_product_not_found() {
        let mut inventory = Inventory::new();
        let mut purchases = Purchases::new();

        // Attempt to record a purchase for a non-existent product
        assert!(purchases
            .record_purchase(&mut inventory, 1, 10, 5.0)
            .is_err());
    }

    #[test]
    fn test_total_purchases() {
        let mut inventory = Inventory::new();
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 100,
        };
        inventory.add_product(product).unwrap();

        let mut purchases = Purchases::new();

        // Record multiple purchases
        purchases
            .record_purchase(&mut inventory, 1, 10, 5.0)
            .unwrap();
        purchases
            .record_purchase(&mut inventory, 1, 5, 2.0)
            .unwrap();

        // Verify the total cost of purchases
        assert_eq!(purchases.total_purchases(), 60.0); // (10 * 5.0) + (5 * 2.0) = 60.0
    }
}

