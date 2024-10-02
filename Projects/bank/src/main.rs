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
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn change_account(account: &mut Account) {
    account.balance = 10;
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn main() {
    let bank = Bank::new();
    let mut account: Account = Account::new(1, String::from("Prathamesh"));

    let account_ref = &account;

    println!("{:?}", bank);

    print_account(&account);

    change_account(&mut account);

    print!("{:#?}", account);
    // print!("{:#?}", account_ref);
}
