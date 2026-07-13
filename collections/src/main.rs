/*
fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut v1 = vec![1, 1, 9];

    v.push(2);
    v.push(8);
    v.push(0);
    v.push(4);
    v1.push(3);

    let third = &v[2];
    println!("The third element in v is {}", third);
    let third1 = v1.get(2);
    if let Some(num) = third1 {
    	println!("The third number in v1 is {}", num);
    }
}

//Remember, you can't have a mutable and an immutable reference in the same scope
fn main() {
	let mut v = vec![1, 2, 3, 4, 5];
	v.push(6);
	let firstval = &v[0];
	if let Some(lastval) = v.last() {
		println!("The last value is {}\nThe first value is {}", lastval, firstval);
	}
}


fn main() {
	let mut scores = vec![17.05, 13.0, 17.9, 11.82, 17.09, 12.];

	println!("{:?}", &scores);
	for score in &mut scores {
		*score = (*score / 20.0) * 400.0;
	}
	println!("{:?}", scores);
}

enum SkinColour {
	Red,
	Green,
	Brown,
	Blue,
	Pink,
	Yellow,
	Black,
	White,
}

fn main() {
	let People = vec![
		SkinColour::Red,
		SkinColour::White,
	];

	
}
*
fn main{
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
*
fn main() {
	let mut s = "Alexander Levi".to_string();
	let mut y = String::from("Geospatial Analysis");

	s.push_str(" 2009");
	y.push_str(" of Nnewi");

	println!("{}\n{}", s, y);
}
*

//Concatenation with + or format! 

// let s1 = String::from("Hello, ");
// let s2 = String::from("Nigga.");
// let s3 = s1 + s2;

fn main() {
// 	let s1 = String::from("tic");
// 	let s2 = String::from("tac");
// 	let s3 = String::from("toe");
// 
// 	let s = s1 + "-" + &s2 + "_" + &s3;

	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	let s = format!("{s1}-{s2}-{s3}");

	println!("{s}");
}
*

fn main() {
	let hello = "Здравствуйте";
//	let answer = &hello[0..5];
//	println!("{}", answer);

	for c in hello.chars() {
		println!("      {c}");
	}
	for b in hello.bytes() {
		println!("      {b}");
	}
}
*/
use std::collections::HashMap;
use std::io;
fn main() {
// 	use std::collections::HashMap;
// 	let mut scores = HashMap::new();
// 
// 	scores.insert(String::from("Sakis"), 18.0);
// 	scores.insert(String::from("Levi"), 15.7);
// 
// 	let score = scores.get("Sakis").copied().unwrap_or(0.0);
// 
// 	println!("Score: {}", score);
// 
// 	for (key, value) in &scores {
// 		println!("{}: {}", key, value);
// 	}

	let mut scores = HashMap::new();
	
	loop {
		let mut keys = String::new();
		let mut values = String::new();
	
		io::stdin().read_line(&mut keys).expect("Not readable device");
		let clean_key =keys.trim().to_string();
		io::stdin().read_line(&mut values).expect("Not readable device");
		let clean_value: i32 = values.trim().parse().expect("please enter a valid num");

		if clean_key ==  "break"{
			break;
		}
		scores.insert(clean_key, clean_value);
		
	}

	for (key, value) in &scores {
		println!("{key}: {value}");
	}
}
