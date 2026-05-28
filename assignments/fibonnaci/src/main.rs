fn main() {
	fibonnaci(10)
}
fn fibonnaci(n: i16) {

	let mut y = 1;
	let mut x = 0;
	let mut z = 0;
	for i in 1..n {
		if i == 2 {z = 1} else {z = y + x}
		x = y;
		y = z;
		println!("fibonacci {} = {}", i, z)
	}
	println!("{z}");
}
