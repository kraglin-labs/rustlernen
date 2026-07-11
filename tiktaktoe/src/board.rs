pub fn showboard(place: &Places) {
	println!("\n     {}  |  {}  |  {}", place.p1, place.p2, place.p3);
	println!("    ________________");
	println!("     {}  |  {}  |  {}", place.p4, place.p5, place.p6);
	println!("    ________________");
	println!("     {}  |  {}  |  {}", place.p7, place.p8, place.p9);
		
}

pub enum User {
	Ux,
	Uy,
}

pub struct Places {
	pub p1: String,
	pub p2: String,
	pub p3: String,
	pub p4: String,
	pub p5: String,
	pub p6: String,
	pub p7: String,
	pub p8: String,
	pub p9: String,
}

pub enum Position{
	P1,
	P2,
	P3,
	P4,
	P5,
	P6,
	P7,
	P8,
	P9,
}

impl Places {
	pub fn new() -> Places {
		Places {
			p1: String::from(" "),
			p2: String::from(" "),
			p3: String::from(" "),
			p4: String::from(" "),
			p5: String::from(" "),
			p6: String::from(" "),
			p7: String::from(" "),
			p8: String::from(" "),
			p9: String::from(" "),
		}		
	}

	pub fn add(&mut self, square: Position, user: User) {
		match user {
			User::Ux => {
				match square {
					Position::P1 => {self.p1 = String::from("X")},
					Position::P2 => {self.p2 = String::from("X")},
					Position::P3 => {self.p3 = String::from("X")},
					Position::P4 => {self.p4 = String::from("X")},
					Position::P5 => {self.p5 = String::from("X")},
					Position::P6 => {self.p6 = String::from("X")},
					Position::P7 => {self.p7 = String::from("X")},
					Position::P8 => {self.p8 = String::from("X")},
					Position::P9 => {self.p9 = String::from("X")},
				}
			}

			User::Uy => {
				match square {
					Position::P1 => {self.p1 = String::from("Y")},
					Position::P2 => {self.p2 = String::from("Y")},
					Position::P3 => {self.p3 = String::from("Y")},
					Position::P4 => {self.p4 = String::from("Y")},
					Position::P5 => {self.p5 = String::from("Y")},
					Position::P6 => {self.p6 = String::from("Y")},
					Position::P7 => {self.p7 = String::from("Y")},
					Position::P8 => {self.p8 = String::from("Y")},
					Position::P9 => {self.p9 = String::from("Y")},
				}
			}
		}
	}
}
