
#[derive(Debug)]
enum IpAddrKind {
    Ipv4,
    Ipv6,
}


fn main() {
    let four = IpAddrKind::Ipv4;
    let six = IpAddrKind::Ipv6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::Ipv4,
        address: "102.89.82.148".to_string(),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::Ipv6,
        address: "::1".to_string(),
    };

    println!("{:?}", home);
    println!("{:?}", loopback);
}

fn route(ip_type: IpAddrKind) {
    println!("{:?}", ip_type)
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}