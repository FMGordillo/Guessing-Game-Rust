
fn main() {
    println!("Please, tell me your name first");
    let mut name = String::new();
    // t adlet mut guess = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("Hello {}nice to meet you", name)
}
