use crate::models::Product;
use std::collections::HashMap;

pub struct Inventory {
    products: HashMap<u32, Product>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) -> Result<(), String> {
        if self.products.contains_key(&product.id) {
            return Err("Product ID already exists".to_string());
        }
        self.products.insert(product.id, product);
        Ok(())
    }

    pub fn edit_product(&mut self, id: u32, new_product: Product) -> Result<(), String> {
        if !self.products.contains_key(&id) {
            return Err("Product not Found".to_string());
        }
        self.products.insert(id, new_product);
        Ok(())
    }

    pub fn delete_prouct(&mut self, id: u32) -> Result<(), String> {
        if !self.products.contains_key(&id) {
            return Err("Product not Found".to_string());
        }
        self.products.remove(&id);
        Ok(())
    }
    pub fn get_product(&self, id: u32) -> Option<&Product> {
        self.products.get(&id)
    }

    pub fn list_products(&self) -> Vec<&Product> {
        self.products.values().collect()
    }
}

// src/inventory.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_product() {
        let mut inventory = Inventory::new();
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 100,
        };
        assert!(inventory.add_product(product).is_ok());
    }

    #[test]
    fn test_add_duplicate_product() {
        let mut inventory = Inventory::new();
        let product = Product {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 10.0,
            quantity: 100,
        };
        inventory.add_product(product.clone()).unwrap();
        assert!(inventory.add_product(product).is_err());
    }
}
