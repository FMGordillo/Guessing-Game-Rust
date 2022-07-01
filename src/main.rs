use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("Please, tell me your name first");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // TODO: Can we improve this?
    let mut rng = rand::thread_rng(); // Necessary, I guess
    let random_float: f64 = rng.gen(); // Float number between 0 and 1
    let random: f64 = random_float.mul_add(100_f64, 0_f64); // Transform in to 1-100 range
    let random_rounded = format!("{}", random.round()); // Round the result
    println!("Hello {}nice to meet you", name);

    println!("And now, tell me your guess");
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    match guess.cmp(&random_rounded) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You've won!"),
    }
    println!("The secret is {}, and your guess was {}", random_rounded, guess);
}
