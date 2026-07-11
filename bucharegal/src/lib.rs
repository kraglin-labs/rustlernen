/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }
    pub mod serving {
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}
fn deliver_order(){}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
        Croissant,
        Meat,
        None,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_food: String,
        pub appetizer: Appetizer,
    }
    
    impl Breakfast {
        pub fn summer(toast: &str, appetizer: Appetizer) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_food: String::from("Peaches"),
                appetizer,
            }
        }
    }


    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
    }
    fn cook_order(){

    }
}

use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    let mut meal = crate::back_of_house::Breakfast::summer("Lasagna", super::back_of_house::Appetizer::Salad);
    meal.toast = "Key Lime Pie".to_string();
}
*/



/* 
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
}
*/

use std::fmt::Result;
use std::io::Result as IoResult; // I used the as keyword to nickname it to avoid conflict
fn main() {
    
}

fn function1() -> Result {}
fn function2() -> IoResult<(), std::io::Error> {}