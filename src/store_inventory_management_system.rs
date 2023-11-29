struct Product {
    name: String,
    description: String,
    price: u64,
    quantity: u64,
  }
  
  impl Clone for Product {
    fn clone(&self) -> Self {
        Product {
            name: self.name.clone(),
            description: self.description.clone(),
            price: self.price,
            quantity: self.quantity,
        }
    }
  }
  struct Transaction {
    product: Product,
    quantity: u64,
    price: u64,
  }
  
  struct InventorySystem {
    inventory: Vec<Product>,
    sales_history: Vec<Transaction>,
    purchase_history: Vec<Transaction>,
  }
  
  struct OutOfStockError;
  
  impl std::fmt::Debug for OutOfStockError {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          write!(f, "Out of stock")
      }
  }
  
  impl std::fmt::Display for OutOfStockError {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
          write!(f, "Out of stock")
      }
  }
  
  impl std::error::Error for OutOfStockError {}
  
  impl InventorySystem {
    fn add_product(&mut self, name: String, description: String, price: u64, quantity: u64) {
        let product = Product { name, description, price, quantity };
        self.inventory.push(product);
    }
  
    fn edit_product(&mut self, index: usize, name: String, description: String, price: u64, quantity: u64) {
        if let Some(product) = self.inventory.get_mut(index) {
            product.name = name;
            product.description = description;
            product.price = price;
            product.quantity = quantity;
        }
    }
  
    fn delete_product(&mut self, index: usize) {
        if index < self.inventory.len() {
            self.inventory.remove(index);
        }
    }
  
    fn record_sale(&mut self, index: usize, quantity: u64, sale_price: u64) -> Result<(), OutOfStockError> {
      if let Some(product) = self.inventory.get_mut(index) {
          if product.quantity >= quantity {
              product.quantity -= quantity;
              let transaction = Transaction {
                  product: product.clone(), // Corrected cloning
                  quantity,
                  price: sale_price,
              };
              self.sales_history.push(transaction);
              Ok(())
          } else {
              Err(OutOfStockError)
          }
      } else {
          // Handle invalid index error
          Err(OutOfStockError)
      }
    }
  
    fn record_purchase(&mut self, index: usize, quantity: u64, purchase_price: u64) {
        if let Some(product) = self.inventory.get_mut(index) {
            product.quantity += quantity;
            let transaction = Transaction {
                product: product.clone(), // Corrected cloning
                quantity,
                price: purchase_price,
            };
            self.purchase_history.push(transaction);
        }
    }
  
    fn generate_report(&self) {
      println!("Inventory Report:");
      println!("{:<20} {:<20} {:<10} {:<10}", "Name", "Description", "Price", "Quantity");
      for product in &self.inventory {
          println!("{:<20} {:<20} {:<10} {:<10}", product.name, product.description, product.price, product.quantity);
      }
  
      println!("\nSales History:");
      println!("{:<20} {:<10} {:<10} {:<10}", "Product", "Quantity", "Sale Price", "Total Sale");
      for transaction in &self.sales_history {
          let total_sale = transaction.quantity * transaction.price;
          println!("{:<20} {:<10} {:<10} {:<10}", transaction.product.name, transaction.quantity, transaction.price, total_sale);
      }
  
      println!("\nPurchase History:");
      println!("{:<20} {:<10} {:<10} {:<10}", "Product", "Quantity", "Purchase Price", "Total Cost");
      for transaction in &self.purchase_history {
          let total_cost = transaction.quantity * transaction.price;
          println!("{:<20} {:<10} {:<10} {:<10}", transaction.product.name, transaction.quantity, transaction.price, total_cost);
      }
    }
  
  }
  
  fn main() {
    let mut inventory_system = InventorySystem {
        inventory: Vec::new(),
        sales_history: Vec::new(),
        purchase_history: Vec::new(),
    };
  
    // Add products
    inventory_system.add_product("Laptop".to_string(), "Powerful laptop".to_string(), 1000, 10);
    inventory_system.add_product("Phone".to_string(), "Smartphone".to_string(), 500, 20);
  
    // Display initial inventory
    println!("Initial Inventory:");
    inventory_system.generate_report();
  
    // Edit a product
    inventory_system.edit_product(0, "High-Performance Laptop".to_string(), "Super powerful laptop".to_string(), 1200, 8);
  
    // Display updated inventory
    println!("\nInventory after Editing Product:");
    inventory_system.generate_report();
  
    // Record a sale
    match inventory_system.record_sale(1, 5, 550) {
        Ok(()) => println!("\nSale recorded successfully"),
        Err(err) => eprintln!("Error: {}", err),
    };
  
    // Display inventory and sales history after the sale
    println!("\nInventory and Sales History after Sale:");
    inventory_system.generate_report();
  
    // Record a purchase
    inventory_system.record_purchase(0, 3, 1100);
  
    // Display inventory and purchase history after the purchase
    println!("\nInventory and Purchase History after Purchase:");
    inventory_system.generate_report();
  
    // Delete a product
    inventory_system.delete_product(1);
  
    // Display final inventory after deleting a product
    println!("\nFinal Inventory after Deleting a Product:");
    inventory_system.generate_report();
  }