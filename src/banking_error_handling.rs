// Define the Account trait
trait Account {
  fn deposit(&mut self, amount: u64) -> Result<(), String>;
  fn withdraw(&mut self, amount: u64) -> Result<(), String>;
  fn balance(&self) -> u64;
}

// Implement the Account trait for the BankAccount struct
struct BankAccount {
  account_number: u32,
  holder_name: String,
  balance: u64,
}

impl Account for BankAccount {
  fn deposit(&mut self, amount: u64) -> Result<(), String> {
      self.balance += amount;
      println!("Deposited {} into account number {}.", amount, self.account_number);
      Ok(())
  }

  fn withdraw(&mut self, amount: u64) -> Result<(), String> {
      if amount <= self.balance {
          self.balance -= amount;
          println!("Withdrawn {} from account number {}.", amount, self.account_number);
          Ok(())
      } else {
          let error_message = format!("Insufficient funds in account number {}.", self.account_number);
          Err(error_message)
      }
  }

  fn balance(&self) -> u64 {
      self.balance
  }
}

fn main() {
  // Create two BankAccount instances
  let mut account1 = BankAccount {
      account_number: 1001,
      holder_name: String::from("Alice"),
      balance: 1000,
  };

  let mut account2 = BankAccount {
      account_number: 1002,
      holder_name: String::from("Bob"),
      balance: 1500,
  };

  // Call deposit on account1
  match account1.deposit(500) {
      Ok(()) => println!("Deposit successful."),
      Err(err) => eprintln!("Deposit error: {}", err),
  }

  // Call withdraw on account2
  match account2.withdraw(200) {
      Ok(()) => println!("Withdrawal successful."),
      Err(err) => eprintln!("Withdrawal error: {}", err),
  }

  // Call balance on both accounts and print the result
  println!("Balance in account {} {}: {}", account1.account_number, account1.holder_name, account1.balance());
  println!("Balance in account {} {}: {}", account2.account_number, account2.holder_name, account2.balance());
}
