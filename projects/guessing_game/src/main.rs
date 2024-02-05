use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number c:");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to readline");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Guess is Smaller"),
            Ordering::Greater => println!("Guess is Greater"),
            Ordering::Equal => {
                println!("Win!!");
                break;
            }
        }
    }
}
