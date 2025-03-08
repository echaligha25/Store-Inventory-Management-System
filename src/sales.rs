use crate::inventory::Inventory;
use crate::models::{Product, Sale};

pub struct Sales {
    sales_history: Vec<Sale>,
}

impl Sales {
    pub fn new() -> Self {
        Sales {
            sales_history: Vec::new(),
        }
    }

    pub fn record_sale(
        &mut self,
        inventory: &mut Inventory,
        product_id: u32,
        quantity_sold: u32,
        sale_price: f64,
    ) -> Result<(), String> {
        if let Some(product) = inventory.get_product(product_id) {
            if product.quantity < quantity_sold {
                return Err("Insufficient stock".to_string());
            }
            let sale = Sale {
                product_id,
                quantity_sold,
                sale_price,
            };
            self.sales_history.push(sale);
            Ok(())
        } else {
            Err("Product not found".to_string())
        }
    }
    pub fn total_sales(&self) -> f64 {
        self.sales_history
            .iter()
            .map(|sale| sale.quantity_sold as f64 * sale.sale_price)
            .sum()
    }
}
