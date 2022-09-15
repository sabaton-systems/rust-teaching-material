fn main() {
    println!("Hello, All!");
    let x = 3;
    match x { //match guard
        3 | 0 => println!("Hello, world!"),
        x if x % 2 == 0 => println!("Hello, Even!"),
        x => println!("Hello, Number!"),
    }
    //for loop
    for num in 1..=10 {
        println!("{}", num);
    }
    
    //while loop
    let mut i = 0;
    while !(i > 9) {
        println!("{}", i);
        i += 1;
    }
    // overflow and macro
    println!["{}",get_number()];
    println!{"{}",get_number()}; 
    println!("{}",get_number());
}

fn get_number() -> i8 {
    return 128;
}