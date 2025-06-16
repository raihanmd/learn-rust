use std::io;

fn main() {
    let secret_number = 42;
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < secret_number {
            println!("Too small!");
        } else if guess > secret_number {
            println!("Too big!");
        } else {
            println!("You win!");
            break;
        }
    }
}

#[test]
fn test() {
    let name: String = String::from("Hello World");
    let age = 20;
    let tall: f32 = 1.8;
    let tallf16 = tall as f64;

    println!(
        "Hello {}, i am {} years old, {} feet tall",
        name, age, tallf16
    );

    println!("Hello {}, i am {} years old, {} feet tall", name, age, tall);
}

#[test]
fn tuple() {
    let mut tuple: (String, u8, f64) = (String::from("Hello World"), 20, 1.8);

    println!("{:?}", tuple);

    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;
    println!("{} {} {}", a, b, c);

    tuple.1 = 30;
    tuple.2 = 1.9;

    let (_, b1, c1) = tuple;
    println!("{} {}", b1, c1);
}

#[test]
fn unit() {
    fn unit() -> () {
        println!("unit");
    }

    let res = unit();
    println!("{:?}", res);

    let unit_pure = ();
    println!("{:?}", unit_pure);
}
