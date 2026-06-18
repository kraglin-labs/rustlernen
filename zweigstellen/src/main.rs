// fn main() {

// 	let s1 = String::from("Hello");
//     let s2 = 4;

//     takes_ownership(s1);
//     makes_copy(s2);
//     println!("{s1}, {s2}");
// } 

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn main(){
//     let s1 = gives_ownership();
//     let s2 = String::from("Yara Yara");
//     let s3 = takes_and_gives_back(s2);

//     println!("{s1}\n{s3}");
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("Phenomenal black woman");
//     some_string
// }

// fn takes_and_gives_back(mut a_string: String) -> String {
//     a_string.push_str(" Is the best phonk");
//     a_string
// }

// fn main(){
//     let s1 = String::from("Hello");
//     let lenn = calculate_length(&s1);
//     println!{"The length of {s1} is {lenn}"};
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("Hello");
//     change(&mut s);
//     println!("{s}")
// }

// fn change(somestring: &mut String){
//     somestring.push_str(", World")
// }

// fn main() {
//     let mut s = String::from("Ohh my zsh");
//     let s1 = &s;
//     let s2 = &s;
//     let s3 = &s;
//     println!("{} {} {}", s1, s2, s3);
//     let s4 = &mut s;
//     println!("{s4}")

// }
// fn main() {
//     let mut s = String::from("Anabella cartez");
//     let word = first_word(&s);
//     println!("the space is after the {}th word", (word - 1));
//     s.clear();
// }


fn main() {
    let s = String::from("Nigga Santa ho ho ho");

    let word = first_word(&s);
    println!("{word}")
}


fn first_words(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, item) in bytes.iter().enumerate(){
        if *item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

