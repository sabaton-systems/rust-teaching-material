fn main() {
   
    println!("MATCH  ....");
    for i in 1..10 {
        match i {
            i if i % 15 == 0 => println!("FizzBuzz"),
            i if i % 5 == 0 => println!("Buzz"),
            i if i % 3 == 0 => println!("Fizz"),
            i => println!("{}", i),
        };
    }
}
