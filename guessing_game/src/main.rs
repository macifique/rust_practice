use std::cmp::Ordering;
use std::io;
use rand::Rng;

const TEST_N: u64 = 3u64;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);

    for _i in 1..=TEST_N {
        println!("Input your guess");
        let mut guessed_number = String::new();

        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Failed to read input");

        println!("You guessed: {}", guessed_number);

        let guessed_number = match guessed_number.trim().parse::<i32>() {
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

    let mut t : [(i32, u32); 2] = [(1,2), (3,4)];

    let mut _t2: [[i32; 3]; 4];

    let _xx = t.get(1);

    fn a(val:i32) -> i32 {
        val+1
    }

    t[0].0 = a(t[1].0);

    let pilincka = &String::new();

    let mut puluncka = pilincka;

    // let ubu: &str = "prima";

    // pilincka.push_str("Hello");
    // pilincka.push_str(" World!");
    //
    // println!("{}", pilincka);
    drop(pilincka.clone());


}
