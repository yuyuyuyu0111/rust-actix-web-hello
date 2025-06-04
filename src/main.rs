use std::io;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    print!("Enter a number>>> ");
    
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: u128 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };
    check_prime(num);
}

fn check_prime(num:u128){
    if num < 2 {
        println!("{} is not a prime number.", num);
        return;
    }
    for i in 2..=((num as f64).sqrt() as u128) {
        if num % i == 0 {
            println!("{} is not a prime number.", num);
            return;
        }
    }
    println!("{} is a prime number.", num);

}
