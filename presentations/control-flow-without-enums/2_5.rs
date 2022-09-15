fn main() {
   
    println!("IF  ....");
    if int_val % 15 == 0 {
        println!("FizzBuzz");
    } else if int_val % 5 == 0 {
        println!("Buzz");
    } else if int_val % 3 == 0 {
        println!("Fizz");
    } else {
        println!("{}", int_val);
    }
}
