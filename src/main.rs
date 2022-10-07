use std::io;

fn main() {
    // 0
    println!("Hello, World!");

    // 1
    let mut guess = String::new();
    println!("Guess the number!");
    println!("Please input your guess below : ");
    io::stdin()
        .read_line(&mut guess)
        .expect("Falied to read line");
    println!("You guessed {guess}");

    // 2
    let apple = 5;
    println!("You have {apple} apples!");
}
