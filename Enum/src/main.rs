use std::io::{self, Read};

#[derive(Debug)]
struct Wallet {
    owner: String,
    balance: f32,
    transactions: Vec<Transaction>,
    new_transaction: Transaction,
    password: String,
}

#[derive(Debug)]
enum Transaction {
    Transfer{name: String, amount: f32},
    Withdraw(f32),
    Deposit(f32),
    None,
}

impl Wallet {
    fn new(owner: &str, password: &str) -> Self {
        Self {
            owner: owner.to_string(),
            balance: 0.0,
            transactions: Vec::new(),
            new_transaction: Transaction::None,
            password: password.to_string(),
        }
    }
    fn transaction(&mut self, trans_type: Transaction) {
        match trans_type {
            Transaction::Withdraw(amount) => {
                let neue = amount + (amount * 0.02);
                loop {
                    let mut pass = String::new();
                    println!("Enter the password:");
                    io::stdin().read_line(&mut pass).expect("Unable to read line");

                    if pass.trim() == self.password {
                        println!("Withdrawing {} with {} charges", amount, neue);
                        withdraw(self, amount);
                        self.transactions.push(Transaction::Withdraw(neue));
                        break;
                    } else {
                        if pass.trim() == "exit" {
                            println!("exiting...");
                            break;
                        } else {
                            println!("Wrong password");
                        }
                    }
                }

            },
            Transaction::Deposit(amount) => {
                let neue = amount - (amount * 0.02);
                loop {
                    println!("Enter the password");
                    let pass = input();
                    if pass.trim() == self.password {
                        println!("Removing 2% charges from {}, depositing {}", amount, neue);
                        deposit(self, neue);
                        self.transactions.push(Transaction::Withdraw(amount));
                        break;
                    } else {
                        if pass.trim() == "exit" {
                            println!("exiting...");
                            break;
                        } else {
                            println!("Wrong password");
                        }
                    }

                }
                
            },
            Transaction::Transfer{name, amount} => {
                let neue = amount + (amount * 0.02);
                loop {
                    println!("Enter the password");
                    let pass = input();
                    if pass.trim() == self.password {
                        if neue < self.balance{
                            println!("Removing 2% charges from {}, Transferring {}", amount, neue);
                            transfer(self, name, neue);
                            self.transactions.push(Transaction::Withdraw(neue));
                            break;
                        } else {
                            println!("Balance not up to {}", neue);
                            continue;
                        }
                        
                    } else {
                        if pass.trim() == "exit" {
                            println!("exiting...");
                            break;
                        } else {
                            println!("Wrong password");
                        }
                    }

                }
                
            },
            _ => (),
        }
    }
}

fn withdraw(person: &mut Wallet, amount: f32) {
    person.balance += amount;
    println!("Successfully withdrawn {} to {}", amount, person.owner);
}
fn deposit(person: &mut Wallet, amount: f32) {
    person.balance += amount;
    println!("Successfully deposited {} to {}", amount, person.owner);
}
fn transfer(person: &mut Wallet, user: String, amount: f32) {
    person.balance -= amount;
    println!("Successfull transferred {} from {} to {} ", amount, person.owner, user)
}
fn input() -> String {
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Unable to read line");
    password
}
fn main() {
    let mut user = Wallet::new("Sakis", "sakis");
    loop {
        turn(&mut user);
    }
    println!("{:#?}", user)
}

fn turn(usr: &mut Wallet) {
    println!("What do you wanna do?");
    let words = input();
    let word: Vec<&str> = words.split_whitespace().collect();
    let neuewort: Vec<String> = word.iter().map(|word| word.to_lowercase()).collect();
    match neuewort[0].as_str() {
        "withdraw" => {
            let amount = neuewort[1].parse().unwrap();
            usr.transaction(Transaction::Withdraw(amount));
        },
        "transfer" => {
            let amount = neuewort[1].parse().unwrap();
            let name = neuewort[3].clone();
            usr.transaction(Transaction::Transfer { name, amount });
        },
        "deposit" => {
            let amount = neuewort[1].parse().unwrap();
            usr.transaction(Transaction::Deposit(amount));
        },
        "show" => {
            match neuewort[1].as_str() {
                "balance" => {
                    println!("{}", usr.balance);
                },
                "history" => {
                    println!("{:?}", usr.transactions);
                },
                "password" => {
                    println!("{}", usr.password);
                }
                _ => {
                    println!("not applicable");
                    println!("You can either:\nshow balance\nshow history\nshow password");
                }
            }
        }
        _ => ()
        
    }
}