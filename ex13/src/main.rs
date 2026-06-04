#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddrKindString {
    V4(String),
    V6(String),
}

// Enhancded version of IpAddrKindString that uses a tuple struct to hold the address data for both V4 and V6 addresses. This allows us to store the address data in a more
#[derive(Debug)]
enum IpAddrKindNumber {
    V4(u8, u8, u8, u8),
    V6(
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
    ),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
struct IpAddrString {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    println!("Hello, world!");

    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let _localhost = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("Home: {:#?}", home);
    println!("Localhost: {:#?}", _localhost);

    let _home_string = IpAddrKindString::V4(String::from("127.0.0.1"));
    let _localhost_string = IpAddrKindString::V6(String::from("::1"));
    println!("Home String: {:#?}", _home_string);
    println!("Localhost String: {:#?}", _localhost_string);

    let _home_number = IpAddrKindNumber::V4(192, 168, 0, 1);
    let _localhost_number = IpAddrKindNumber::V6(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1);
    println!("Home Number: {:#?}", _home_number);
    println!("Localhost Number: {:#?}", _localhost_number);
}

fn route(ip_kind: IpAddrKind) {
    // --snip--
    println!("Routing {:?}", ip_kind);
}
