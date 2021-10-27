/*enum IpAddrKind {
    v4,
    v6,
}

struct IpAddr {
    kind:IpAddrKind,
    address:String,
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::v6,
        address:String::from("::1"),
    };
}
*/


/*enum IpAddr {
    v4(String),
    v6(String),
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.01"));

    let loopback = IpAddr::v6(String::from("::1"));
}*/


enum IpAddr {
    v4(u8,u8,u8,u8),
    v6(String),
}

fn main() {
    let home = IpAddr::v4(127,0,0,1);
    let loopback = IpAddr::v6(String::from("::1"));
}