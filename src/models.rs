//struct for Product data type details
#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: u32,
}
//struct for Sales data type details
#[derive(Debug)]
pub struct Sale {
    pub product_id: u32,
    pub quantity_sold: u32,
    pub sale_price: f64,
}

//struct for Sales data type details
#[derive(Debug)]
pub struct Purchase {
    pub product_id: u32,
    pub quantity_purchased: u32,
    pub purchase_price: f64,
}
