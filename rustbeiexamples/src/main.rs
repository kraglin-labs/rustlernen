// fn main() {
//     let (x, y) = (1, 2);
//     let s = sum(x, y);
//     assert_eq!(s, 3);
//     println!("Success");
// }
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn main() {
//     print();
// }
// fn print(){
//     println!("Success");
// }

fn main(){
    never_returns3();
    println!("Failed");
}

// fn never_returns1() -> ! {
//     println!("Success");
//     std::process::exit(1)
// }
// fn never_returns2() -> !{
//     loop{

//     }
// }
fn never_returns3() -> !{
    panic!("Success");
}

