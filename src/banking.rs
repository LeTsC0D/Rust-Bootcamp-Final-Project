// Define the Account trait
trait Account {
  fn deposit(&mut self, amount: u64);
  fn withdraw(&mut self, amount: u64);
  fn balance(&self) -> u64;
}

// Implement the Account trait for the BankAccount struct
struct BankAccount {
  account_number: u32,
  holder_name: String,
  balance: u64,
}

impl Account for BankAccount {
  fn deposit(&mut self, amount: u64) {
      self.balance += amount;
      println!("Deposited {} into account number {}.", amount, self.account_number);
  }

  fn withdraw(&mut self, amount: u64) {
      if amount <= self.balance {
          self.balance -= amount;
          println!("Withdrawn {} from account number {}.", amount, self.account_number);
      } else {
          println!("Insufficient funds in account number {}.", self.account_number);
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
  account1.deposit(500);

  // Call withdraw on account2
  account2.withdraw(200);

  // Call balance on both accounts and print the result
  println!("Balance in account {}: {}", account1.account_number, account1.balance());
  println!("Balance in account {}: {}", account2.account_number, account2.balance());
}
