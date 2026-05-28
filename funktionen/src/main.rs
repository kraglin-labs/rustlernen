fn main(){
    another_funktionen(55, 'm');
    let five = five();
    println!("The number is {five}")
}

fn another_funktionen(value: i32, unit: char){
    println!("The measurement is {value}{unit}");
}

fn five() -> i32 {
    5
}
