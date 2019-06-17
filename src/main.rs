
use std::io;

fn main() {
    let guess = 5;
    println!("I have picked a number.");

    loop {
        println!("Try a guess : ");
        let mut user = String::new();
        io::stdin()
            .read_line(&mut user)
            .expect("Failed to read line");
        let user: i32 = user.trim().parse().expect("Please type a number!"); //32bit integer, 32bit unsigned number

        println!("You guessed {}", user);

        if user < guess {
            println!("Too small!");
        } else if user > guess {
            println!("Too big!");
        } else {
            println!("You win!");
            break;
        }
    }
}
