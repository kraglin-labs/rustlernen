// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6,
//     }

//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }

//     let _four: IpAddrKind = IpAddrKind::V4;
//     let _six: IpAddrKind = IpAddrKind::V6;


//     fn route(_ip_kind: IpAddrKind) {
        
//     }

//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);

// }   


// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// fn main(){
//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: "127.0.0.1".to_string(),
//     };
//     let loop_back = IpAddr {
//         kind: IpAddrKind::V6,
//         address: "::1".to_string(),
//     };



// }

// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let home = IpAddrKind::V4(String::from("127.0.0.1"));
//     let loopback = IpAddrKind::V6(String::from("::1"))
// }
#[allow(unused)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let _home = IpAddrKind::V4(127, 0, 0, 1);
    let _loopback = IpAddrKind::V6(String::from("::1"));
}