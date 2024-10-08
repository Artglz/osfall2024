fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}
use std::io;
fn main() {
    let secret_number = 7;
    let mut attempts = 0;

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");        
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        attempts += 1;

        let result = check_guess(guess, secret_number);

        if result == 0 {
            println!("Correct! The secret number is {}", secret_number);
            break;
        } else if result == 1 {
            println!("Too high");
        } else {
            println!("Too low");
        }
    }

    println!("It took {} attempts", attempts);
}

