pub mod accounts;
pub mod transactions;

pub fn announce(message: &str) {
	println!(
		"[BANK ANNOUNCMENT] {}",
		message
	)
}
