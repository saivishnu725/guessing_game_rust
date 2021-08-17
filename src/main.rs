use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    let random_number = rand::thread_rng().gen_range(1..=10);
    loop {
        println!("Guess the number! ");
        println!("Input your guess. ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Please give a string");
        // println!("Secret number is {} ", random_number);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter only numbers !!!\n\n\n\n");
                continue;
            }
        };
        match guess.cmp(&random_number) {
            Ordering::Equal => {
                println!("You win!!\n");
                break;
            }
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
        }
        println!("\n\n\n\n");
    }
}
