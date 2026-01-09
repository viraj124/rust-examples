use std::io;
use std::cmp::Ordering;

use rand::Rng;

pub(crate) fn checkRandom() {
    println!("Guess the number!");

    let rand = rand::thread_rng().gen_range(0..=100);


    loop {
    println!("Please input your guess.");

    let mut guess = String::new();
    

    io::stdin().read_line(&mut guess)
    .expect("failed to read");

    println!("you choose {guess}");
    
    // shadowing the existing guess variable so the previous guess cannot be used again
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    // &guess is a reference so that we can read but cant modify, immutable by default in rust
    match rand.cmp(&guess) {
        Ordering::Less => println!("wrong less than"),
        Ordering::Greater => println!("wrong more than"),
        Ordering::Equal => {
            println!("winner");
            break;
        }
    }
    }
}