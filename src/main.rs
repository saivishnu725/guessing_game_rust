use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number! ");
    println!("Input your guess. ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Please give a string");
    println!("it is : {}", guess);
    println!("Secret number is {} ", random_number);
    let guess: i32 = guess.trim().parse().expect("Please type a number!");

    match guess.cmp(&random_number) {
        Ordering::Equal => println!("You win!!"),
        Ordering::Less => println!("Too small!!"),
        Ordering::Greater => println!("Too big!!"),
    }
}
