use std::io;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::thread;
use std::time::Duration;
use std::process;

fn main() {
    println!("Give me your best guess 1 - 1 000 000 000 :)");

    let secret_number = rand::thread_rng().gen_range(1..=1000000000);

    let guess_high: [&str; 10] = [
        "Too high! Try a lower number.",
        "Nope, that's too big! Aim lower.",
        "That's a bit too much! Lower your guess.",
        "You're overshooting it! Try something smaller.",
        "That's not quite right, it's too high.",
        "Your guess is too high, dial it down a bit!",
        "Whoa, too big! Go lower!",
        "That's way too much! Aim for a smaller number.",
        "You're getting warmer, but that's still too high!",
        "Almost there, but you're still too high!"
    ];

    let guess_low: [&str; 10] = [
        "Too low! Try a higher number.",
        "Nope, that's too small! Aim higher.",
        "That's a bit too little! Raise your guess.",
        "You're undershooting it! Try something bigger.",
        "That's not quite right, it's too low.",
        "Your guess is too low, crank it up a bit!",
        "Whoa, too small! Go higher!",
        "That's way too little! Aim for a bigger number.",
        "You're getting warmer, but that's still too low!",
        "Almost there, but you're still too low!"
    ];

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
            Ordering::Less => println!("{}", guess_low.choose(&mut thread_rng()).unwrap()),
            Ordering::Greater => println!("{}", guess_high.choose(&mut thread_rng()).unwrap()),
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
