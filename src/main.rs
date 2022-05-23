use rand::prelude::*;

fn main() {
    println!("Please, tell me your name first");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // TODO: Can we improve this?
    let mut rng = rand::thread_rng(); // Necessary, I guess
    let random_float: f64 = rng.gen(); // Float number between 0 and 1
    let random: f64 = random_float.mul_add(10_f64, 0_f64); // Transform in to 1-10 range
    let random_rounded: f64 = random.round(); // Round the result
    println!("Hello {}nice to meet you", name);
    println!("My number {}", random_rounded);
}
