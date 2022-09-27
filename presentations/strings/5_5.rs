struct Point(i32, i32, String);

fn main() {
    let p = Point(1, 2, String::from("Five"));
    println!("{}", p.0);
    println!("{}", p.1);
    println!("{}", p.2);
}