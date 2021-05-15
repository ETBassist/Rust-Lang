fn main() {
    //let four = IpAddrKind::V4; // kinds are namespaced under enum identifier
    //let six = IpAddrKind::V6;

    let home = IpAddr::V4(IpAddrV4 { address: (127, 0, 0, 1) });
    home.display_ip();

    let loopback = IpAddr::V6(IpAddrV6{ address: String::from("::1") });
    loopback.display_ip();

    println!("The value of a penny is {}", value(Coin::Penny));
    println!("The value of a nickel is {}", value(Coin::Nickel));
    println!("The value of a dime is {}", value(Coin::Dime));
    println!("The value of a quarter is {}", value(Coin::Quarter));
}


// defining a function with IpAddrKind as type in arg allows acceptance of
// either IpAddrKind::V4 or IpAddrKind::V6
fn _route(_ip_addr: IpAddr) {} // temp underscores to ignore compiler warnings

// you can put any kind of data in an enum variant (including other enums)
enum IpAddr {
    V4(IpAddrV4),
    V6(IpAddrV6),
}

// like a struct, can define functions in impl block for an enum
impl IpAddr {
    fn display_ip(&self) {
        match &self {
            IpAddr::V6(ip_v6) => println!("The ip is: {}", ip_v6.address),
            IpAddr::V4(ip_v4) => println!("The ip is: {}.{}.{}.{}", ip_v4.address.0, ip_v4.address.1, ip_v4.address.2, ip_v4.address.3),
        }
    }
}

struct IpAddrV4 {
    address: (u8, u8, u8, u8),
}

struct IpAddrV6 {
    address: String,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value(coin: Coin) -> u8 { // match enum and return value dependent on sub type
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
