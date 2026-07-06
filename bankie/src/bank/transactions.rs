use crate::bank::accounts::Account;

pub fn deposit(acc: &mut Account, amount: i32) {
	acc.balance += amount;
	println!(
		"[Transaction] Deposited ${}. new balance ${}",
		amount, acc.balance
	);
}

pub fn withdraw(acc: &mut Account, amount: i32) {
	if amount < acc.balance {
		println!(
			"[Transaction] ERROR: Insufficient funds for ${} withdrawal",
			amount
		)
	} else {
		acc.balance -= amount;
		println!(
			"[TRANSACTION] Withdraw ${}, New balance: ${}",
			amount, acc.balance,
		)
	}
}
