use std::cmp::Ordering;
use std::io;
use rand::Rng;

// Based on https://doc.rust-lang.org/book

fn main() {
    //chapter1();
    //chapter2();
    //chapter3_1();
    chapter3_2();
}

#[allow(dead_code)]
fn chapter1() {
    println!("Hello World!");
}

#[allow(dead_code)]
fn chapter2() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a valid number!");
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[allow(dead_code)]
fn chapter3_1() {
    // mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

#[allow(dead_code)]
fn chapter3_2() {
    let x = 0.1 + 0.2;
    println!("{x}");
}
