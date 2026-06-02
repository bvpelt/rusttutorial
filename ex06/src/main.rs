fn main() {
    println!("Hello, world!");

    let _t: i32 = 5;
    let _s: &i32 = &_t; // _s immutable reference to _t, ownership is moved

    println!("_t: {}, _s: {}", _t, _s);

    let mut _x: i32 = 7;
    let _r: &mut i32 = &mut _x; // _r mutable reference to _x, ownership is moved

    *_r += 1;

    println!("_t: {}, _s: {}, _x: {}", _t, _s, _x);

    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // immutable borrow to check the balance
    account.check_balance();

    // mutable borrow to withdraw money
    account.withdraw(45.50);

    // immutable borrow to check the balance
    account.check_balance();
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: String, balance: f64) -> Self {
        BankAccount { owner, balance }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        println!(
            "Attempting to withdraw ${} from {}'s account",
            amount, self.owner
        );
        if self.balance >= amount {
            self.balance -= amount;
        } else {
            println!("Insufficient funds");
        }
    }

    fn check_balance(&self) -> f64 {
        println!(
            "Account owner: {}, balance: ${:.2}",
            self.owner, self.balance
        );
        return self.balance;
    }
}
