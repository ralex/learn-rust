enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

// Options<T> is defined by the standard library as follows:
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    route(IpAddrKind::V4(127, 0, 0, 1));
    route(IpAddrKind::V6(String::from("::1")));

    let m = Message::Write(String::from("Bye!"));
    m.call();

    let _some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;
}

fn route(_ip_kind: IpAddrKind) {}
