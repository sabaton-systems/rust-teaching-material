fn main() {
    println!("Hello, All!");
    let x = 3;
    match x { //match guard
        3 | 0 => println!("Hello, world!"),
        x if x % 2 == 0 => println!("Hello, Even!"),
        x => println!("Hello, Number!"),
    }
}