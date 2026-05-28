fn main() {
	let val = 20;
	let unit = 'c';

	
	let opp = if unit == 'c' {'f'} else if unit == 'C' {'F'} else if unit == 'f' {'c'} else if unit == 'F' {'c'} else {' '};
	println!("{}°{} is {}°{}", val, unit, convert(val, unit), opp)
}

fn to_fahrenheit(celcius: i32) -> i32 {
	(celcius * 9/5) + 32
}

fn to_celcius(fahrenheit: i32) -> i32 {
	(fahrenheit - 32) * 5/9
}

fn convert (value: i32, unit: char) -> i32 {
	//let nunit = char::to_lowercase(unit);
	if unit == 'f' || unit == 'F' {
		to_celcius(value)
	} else if unit == 'c' || unit == 'C'{
		to_fahrenheit(value)
	} else {
		println!("enter either F or C");
		0
	}
}
