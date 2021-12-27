use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Input your guess");
        let mut guessed_number = String::new();

        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Failed to read input");

        println!("You guessed: {}", guessed_number);

        let guessed_number: i32 = match guessed_number.trim().parse() {
            Ok(num) => num,
            _ => continue,
        };

        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("MATCH!");
                break;
            },
            Ordering::Greater => println!("Too big!")
        }

    }

}
