use std::{cmp::Ordering, io}; 
use rand::Rng; 

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100); 

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new(); 
    
        io::stdin()
            .read_line(&mut guess) 
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}! The secret number was {secret_num}");
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Smallio!"), 
            Ordering::Greater => println!("Largeio!"), 
            Ordering::Equal => {
                println!("Winner!");
                break;
            },
        }
    }
}   
