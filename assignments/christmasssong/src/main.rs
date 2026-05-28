fn main() {
    println!("Hello, world!");
}

fn christmass_song() {
	let array = [
		"A patridge in a pear tree",
		"Two turtle doves and",
		"Three french hens",
		"Four calling birds",
		"Five golden rings",
		"Six geese are laying",
		"Seven swarms are swimming",
		"Eight maid are milking",
		"Nine ladies dancing",
		"Ten lords are leaping",
		"Eleven pipers piping",
		"Twelve drummers drumming",
	];

	let i: i32 = 0;
	for i  in 1..=12 {
		println!("on the {} day of christmass my true love gave to me", numero(i));
		i += 1;
		println!("{}", array[i - 1]);
	}
}

fn numero(n: u16) -> String {
	match n {
		1 => String::from("first"),
		2 => String::from("second"),
		3 => String::from("third"),
		4 => String::from("fourth"),
		5 => String::from("fifth"),
		6 => String::from("sixth"),
		7 => String::from("seventh"),
		8 => String::from("eigth"),
		9 => String::from("ninth"),
		10 => String::from("tenth"),
		11 => String::from("eleventh"),
		12 => String::from("twelvth"),
		_ => String::from("nada"),
	}
	
}
