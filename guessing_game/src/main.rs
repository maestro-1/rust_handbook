use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number");

    let correct_guess = rand::thread_rng().gen_range(1..101);

    println!("The correct guess is: {}", correct_guess);

    loop {
        println!("Please input your guess");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("your guess is {}", guess);
    
        match guess.cmp(&correct_guess) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win".green());
                break;
            },
        }   
    }
}
