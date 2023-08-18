fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;

    println!("{x} * {y} = {}", multiply(x.into(), y));

    let a: bool = false;
    let b: &str = "42";

    println!("{a} * {b} = {}", multiply(a.into(), b.parse().unwrap()));

    let c: f32 = 1.1;
    let d: i128 = i16::max_value().into(); // the program should panic if we choose a value above i16 max (32767)

    println!("{c} * {d} = {}", multiply(c.round() as i16, d as i16));
}
