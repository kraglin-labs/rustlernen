
// struct IpV4Addr() {

// }
// struct IpV6Addr() {

// }
// enum IpAddr {
//     V4(IpV4Addr),
//     v6(IpV6Addr),
// }
// fn main(){

// }
//This 
// enum Game {
//     Move(x: i32, y: i32),
//     Quit,
//     Write(String),
//     ChangeColour(i32, i32, i32),
// }
// //Is the same thing as this:
// struct Move{
//     x: i32,
//     y: i32,
// }
// struct Quit; //This ist a unit struct
// struct ChangeColour(i32, i32, i32); // Tuple struct
// struct Write(String); //Tuple struct also
//There can also be methods:
// #[derive(Debug)]
// enum Game {
//     Move{x: i32, y: i32},
//     Quit,
//     Write(String),
//     ChangeColour(i32, i32, i32),
// }

// impl Game {
//     fn call(&self) {
//         let mut neue = Vec::new();
//         neue.push(&self);

//         for neu in neue {
//             println!("{:?}", neu);
//         }
//     }
// }

// fn main() {
//     let m = Game::Move{x: 3, y: 4};
//     m.call();
// }

//the option enum

// enum Option<T> {
//     None,
//     Some(T),
// }

// fn main() {
//     let number = Some(5);
//     let name = Some("Sakis");
//     let nonexistent_number: Option<i32> = None;

//     match number {
//         Some(value) => println!("Value: {}", value),
//         None => println!("Value: None"),
//     }
//     match name {
//         Some(value) => println!("Value: {}", value),
//         None => println!("Value: None"),
//     }
//     match nonexistent_number {
//         Some(value) => println!("Value: {}", value),
//         None => println!("Value: None"),
//     }
// }

// #[allow(dead_code)]
// enum Coins {
//     Penny(u32),
//     Nickel(u32),
//     Dime(u32),
//     Quarter(u32),
//     Dollar(u32),
// }

// fn value_in_cents(coin: Coins) -> u32 {
//     match coin {
//         Coins::Penny(val) => 1 * val,
//         Coins::Nickel(val) => 5 * val,
//         Coins::Dime(val) => 10 * val,
//         Coins::Quarter(val) => 25 * val,
//         Coins::Dollar(val) => 100 * val,
//     }
// }

// fn main() {
//     let nummer: u32 = 3219;
//     let value = "n";
//     let buck: Coins = {
//         match value {
//             "p" => Coins::Penny(nummer),
//             "n" => Coins::Nickel(nummer),
//             "d" => Coins::Dime(nummer),
//             "q" => Coins::Quarter(nummer),
//             "dl" => Coins::Dollar(nummer),
//             _ => Coins::Dime(nummer),
//         }
//     };
//     println!("{} cents", value_in_cents(buck));
// }

// fn main(){
//     let config_max = Some(3u8);
//     match config_max {
//         Some(max) => {println!("Max value: {max}")},
//         _ => (),
//     }

// }

// fn main(){
//     let config_max = Some(3u8);

//     if let Some(maximum) = config_max {
//         println("The max is {}", maximum);
//     }
// }
#[derive(Debug)]
#[allow(dead_code)]
enum SchoolMember {
    Student {name: String, cgpa: f32},
    Teacher {name: String, id: String},
    RegularStaff {name: String, id: String},
}

#[allow(dead_code)]
fn get_info_lame(person: SchoolMember) -> (Option<String>, Option<f32>) {
    if let SchoolMember::Student{name, cgpa} = person {
        return (Some(name), Some(cgpa))
    } else {
        return (None, None);
    };
}

fn get_info_cool(person: SchoolMember) -> (Option<String>, Option<f32>) {
    let SchoolMember::Student{name, cgpa} = person else {
        return (None, None);
    };
    (Some(name), Some(cgpa))
}
fn main() {
    let person1 = SchoolMember::Student{name: "Damien".to_string(), cgpa: 4.7f32};
    let person2 = SchoolMember::Teacher{name: "Alek".to_string(), id: "39151f".to_string()};
    
    if let (Some(name), Some(grade)) = get_info_cool(person2) {
        println!("{} has GPA: {}", name, grade);
    } else {
        println!("Not a student");
    }
    
}