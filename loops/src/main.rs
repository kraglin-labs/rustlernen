fn main() {
    /*loop {
    	println!("Again");
    }
    let x = 30;
	println!("{} factorial is {}", x, factorial_calc(x));
	let mut counting = 0;

	'counting_up: loop {
		println!("count = {counting}");
		let mut remaining = 10;

		loop {
			println!("remaining = {remaining}");

			if remaining == 9 {
				break;
			}

			if counting == 2 {
				break 'counting_up;
			}

			remaining -= 1;
		}
		counting += 1;
	}
	println!("end count = {counting}");

	let mut number = 5;
	while number != 0 {
		println!("{number}");
		number -= 1;
	}
	println!("Kaboom!!")*/

	/*let array = [10, 20, 30, 40, 50];
	let mut size = array.len();
	while size > 0 {
		println!("the value of index {} is {}", size-1, array[size - 1]);
		size -= 1
	}
	let mut index = 0;
	for arr in array {
		println!{"The value of index {} is {}", index, arr};
		index += 1;
	}*/

	for number in (1..=5).rev() {
		println!("{}", number);
	}
	println!("blast off")
}

/*fn factorial_calc(n: i128) -> i128{
	let mut counter: i128 = 0;
	let mut factor: i128 = 1;

	let factorial: i128 = loop {
		counter += 1;
		factor *= counter;

		if counter > n - 1 {
			break factor;
		}
	};
	factorial
}*/
