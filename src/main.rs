enum IpAddrKind{
    V4(String),
    V6(String),
}

fn routes(ip_kind: IpAddrKind) {

}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(String::from("127.0.0.1"))
    let loopback = IpAddrKind::V6(String::from("::1"))

}
