// #[derive(Debug)]
// struct User {
//     name: String,
//     email: String,
//     is_active: bool,
//     age: u8,
// }

// fn main() {
//     let user1 = User {
//         name: String::from("Johnie Cruz"),
//         email: String::from("johnieboy@gmail.com"),
//         is_active: true,
//         age: 19,
//     };

//     let user2 = User {
//         name: String::from("Anabella Cortez"),
//         email: String::from("anabella@gmail.com"),
//         is_active: user1.is_active,
//         age: user1.age,
//     };

//     println!("{:#?}", user2);
//     println!("{:#?}", user1);

// }

// fn build_user(name: String, email: String, is_active: bool, age: u8) -> User {
//     User {
//         name,
//         email,
//         is_active,
//         age,
//     }
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(3, 9, 1);

//     println!("Black: ");
//     println!("R: {}\nG: {}\nB: {}", black.0, black.1, black.2);
//     println!("Origin: ");
//     println!("X axis: {}\nY axis: {}\nZ axis: {}", origin.0, origin.1, origin.2);
// }

//Unit structures
// struct AlwaysEqual;

// fn main() {
//     let subject = AlwaysEqual;
// }

// fn main() {
//     let rectangle1 = (34, 41);

//     let area = area(rectangle1);

//     println!("The area of the rectangle is {}", area)
// }

// fn area(dimens: (i32, i32)) -> i32 {
//     dimens.0 * dimens.1
// }
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 22,
//         height: 19,
//     };

//     let area = area(&rect1);

//     println!("Area: {}", area);
//     println!("Rectangle 1 is {:?}", rect1)
// }

// fn area (rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

// const PI: f32 = 3.14159;

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// struct Circle {
//     radius: f32,
//     circumfrence: f32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//     fn perimeter(&self) -> u32 {
//         (self.width *2) + (self.height * 2)
//     }
// }

// impl Circle {
//     fn area(&self) -> f32 {
//         PI * self.radius * self.radius
//     }
// }














// fn main () {
//     let rect1 = Rectangle {
//         width: 33,
//         height: 81,
//     };

//     let mut circa1 = Circle {
//         radius: 5.4,
//         circumfrence: 1.0,
//     };

//     circa1.circumfrence = PI * (2.0 * circa1.radius);
//     println!("The area of the rectangle is {}", rect1.area());
//     println!("The perimeter of the rectangle is {}", rect1.perimeter());
//     println!("The area of the circle is {}", circa1.area());
//     println!("The circumfrence of the circle is {}", circa1.circumfrence)
// }


// struct Humane {
//     height: f32,
//     mass: f32,
//     name: String
// }

// impl Humane {
//     fn bmi(&self) -> f32 {
//         self.mass / (self.height * self.height)
//     }
// }

// fn main() {
//     let jason = Humane {name: "Jason".to_string(), height: 1.83, mass: 56.0};
//     println!("{}'s bmi = {}", jason.name, jason.bmi());
// }

struct Rectangle {
    width: f32,
    height: f32,
    area: f32,
}

impl Rectangle {
    fn area(&self) -> f32{
        self.width * self.height
    }
    fn can_fit(&self, neue: &Rectangle) -> bool {
        self.width > neue.width && self.height > neue.height
    }

    fn square(size: f32) -> Self {
        Self {
            width: size,
            height: size,
            area: size * size,
        }
    }
}

fn main() {
    let mut rect1 = Rectangle {width: 23.4, height: 30.0, area: 0.0};
    let mut rect2 = Rectangle {width: 24.0, height: 19.9, area: 0.0};

    rect1.area = rect1.area();
    rect2.area = rect2.area();

    if rect1.area > rect2.area {
        println!("Rect 1 is bigger than Rect 2");
        if rect1.can_fit(&rect2) {
            println!("Rect 2 can fit Rect 1");
        }
        else {
            println!("Rect 2 cannot fit in Rect 1");
        }
    }
    else if rect1.area < rect2.area {
        println!("Rect 2 is bigger than Rect 1");
    }

    let neue = Rectangle::square(9.0);
    println!("The area of neue is {}", neue.area)
}