use std::io;
use std::io::Write;     // for .flush(), unsure why ::Read gets pulled in automatically but not ::Write...
use std::cmp::Ordering;

fn main() {
    let secret = rand::random_range(1..=100);
    // println!("random number: {secret}");

    loop {
        let mut guess = String::new();

        print!("input guess or \"exit\": "); io::stdout().flush().expect("failed to flush");
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read");
        let guess = match guess
            .trim()
            .parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    if guess.trim() == "exit" { break };
                    continue;
                },
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("just right!");
                break;
            },
        }
    }
}
