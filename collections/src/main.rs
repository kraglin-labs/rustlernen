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
*/
//Remember, you can't have a mutable and an immutable reference in the same scope
// fn main() {
// 	let mut v = vec![1, 2, 3, 4, 5];
// 	v.push(6);
// 	let firstval = &v[0];
// 	if let Some(lastval) = v.last() {
// 		println!("The last value is {}\nThe first value is {}", lastval, firstval);
// 	}
// }
