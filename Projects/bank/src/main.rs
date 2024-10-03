#[derive(Debug)]
struct Account {
    balance: u32,
    id: u32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: u32) -> u32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: u32) -> u32 {
        self.balance -= amount;
        self.balance
    }

    fn summary(&self) -> String {
        format!(
            "Account ID: {}, Holder: {}, Balance: {}",
            self.id, self.holder, self.balance
        )
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> u32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();
    let mut account: Account = Account::new(10, String::from("Prathamesh"));

    account.deposit(200);
    account.withdraw(100);

    bank.add_account(account);

    println!("{:#?}", bank.summary());
    println!("{:#?}", bank.total_balance());
}
