const MAX_POINTS: u32 = 100000;
use std::io;
fn prt() {
    let a = 12;
    println!("a is {0} {0} a is {0} {1}", a, MAX_POINTS);
}

fn main(){
    println!("Guess the number!");

    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}\n", guess);
}