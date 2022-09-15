fn main() {
   
    println!("MAP and MATCH ....");
    let numbers = (0..10).map(|x| match x {
        x if x % 15 == 0 => String::from("FizzBuzz"),
        x if x % 5 == 0 => String::from("Buzz"),
        x if x % 3 == 0 => String::from("Fizz"),
        x => format!("{}", x),
    });

    for i in numbers {
        println!("{}", i);
    }
}
