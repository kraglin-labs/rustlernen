#[derive(Debug)]
pub struct Account {
	pub owner: String,
	pub balance: i32,
}

impl Account{
	pub fn new(owner: &str) -> Account{
		Account  {
			owner: owner.to_string(),
			balance: 0,
		}
	}
}
