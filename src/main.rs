use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::thread;
use std::time::Duration;
use std::process;

fn main() {
    println!("Give me your best guess 1 - 1 000 000 :)");

    let secret_number = rand::thread_rng().gen_range(1..=1000000);

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input must be a number!");
                continue;
            }
        };

        println!("You gessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    println!("The app will close in 10 seconds...");

    thread::sleep(Duration::from_secs(10));

    process::exit(0);
}
