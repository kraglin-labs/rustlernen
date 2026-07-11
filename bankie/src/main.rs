/*
	renamed bank.rs to mod.rs
*/

mod bank ;

fn main() {
	let mut account = bank::accounts::Account::new("Sakis");
	println!("[ACCOUNT] created: {:?}", account);

	bank::transactions::deposit(&mut account, 390);
	bank::transactions::deposit(&mut account, 470);

	println!("[ACCOUNT] Final state: {:?}", account);

	bank::announce("There is a maintainance at 1:30");
}
