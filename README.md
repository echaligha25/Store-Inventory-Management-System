# Rusty Store Inventory Management System

## Overview

The **Rusty Store Inventory Management System** is a command-line application designed to help small retail stores manage their inventory, sales, and purchases efficiently. This system is built using the Rust programming language and demonstrates the use of core Rust concepts such as ownership, borrowing, structs, enums, traits, and error handling.

## Features

1. **Inventory Management**  
   - Add, edit, and delete products.  
   - Track product name, description, price, and quantity.  

2. **Sales Management**  
   - Record sales transactions (product, quantity sold, sale price).  
   - Calculate and display total sales and profit for each transaction.  

3. **Purchase Management**  
   - Record purchase transactions (product, quantity purchased, purchase price).  
   - Calculate and display the total cost of each purchase.  

4. **Reporting**  
   - Generate inventory, sales, and purchase history reports.  
   - Reports are displayed in a user-friendly, well-structured format.  

5. **Error Handling**  
   - Handle invalid inputs, out-of-stock errors, and other edge cases gracefully.  

6. **Security**  
   - Basic authentication to restrict unauthorized access to the system.  

7. **User Interface**  
   - Text-based UI with simple and intuitive navigation for store managers.  

## Prerequisites

- Install [Rust](https://www.rust-lang.org/) (minimum version 1.65).  
- Install a SQLite database client if you plan to store data persistently (optional).

## Installation

1. Clone this repository:  
   ```bash
   git clone https://github.com/echaligha25/tore-Inventory-Management-System.git
   cd rusty-store-inventory
