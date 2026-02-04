#[derive(Debug)]
struct BankAccount {
    balance: f64,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited ₦{:.2}. New balance: ₦{:.2}", amount, self.balance);
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
            println!("Withdrew ₦{:.2}. New balance: ₦{:.2}", amount, self.balance);
        } else {
            println!("Insufficient funds! Current balance: ₦{:.2}", self.balance);
        }
    }

    fn check_balance(&self) {
        println!("Current balance: ₦{:.2}", self.balance);
    }
}

fn main() {
    let mut account = BankAccount {
        balance: 1000.0,
    };

    account.check_balance();
    account.deposit(500.0);
    account.withdraw(200.0);
    account.withdraw(2000.0);
}