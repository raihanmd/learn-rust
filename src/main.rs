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

#[test]
fn array() {
    let mut my_array: [u8; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", my_array);

    my_array[0] = 0;

    let [a, b, _, _, _] = my_array;
    let c = my_array[2];
    println!("{} {} {}", a, b, c);

    println!("array length: {}", my_array.len());

    assert!(a == 0);
}

#[test]
fn two_dimensional_array() {
    const MAXIMUM: i32 = 20;

    let matrix: [[u8; 2]; 2] = [[1, 2], [4, 5]];

    println!("{:?}, {}", matrix, MAXIMUM);
}

//? Stack and Heap concept memory management rust

//* setiap type data yg fixed, ketika di ubah / mutable, data sebelumnya akan tetap ada, itu hanya mengubah apa yg dipegang sama si variable
//* Cukup berpikir seperti ada return value, dan nanti hasilnya harus di assignkan lagi

#[test]
fn str_slice() {
    let str = "Hello World\r\n";

    let trimmed = str.trim();

    println!("{} {}", str, trimmed);

    assert_eq!(trimmed, "Hello World");
}

//* the 'String' type is a growable, UTF-8 encoded data type, so its stored on Heap

#[test]
fn string_type() {
    let mut name: String = String::from("Hello");
    name.push_str(" World");

    println!("{}", name);

    let sparkle_heart = vec![240, 159, 146, 150];

    // We know these bytes are valid, so we'll use `unwrap()`.
    let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();

    println!("{}", sparkle_heart);
    assert_eq!("💖", sparkle_heart);
}

#[test]
fn ownership() {
    let a: i32 = 200;

    let mut b = a; //* its just copy from value a to b

    b += 100;

    println!("{} {}", a, b);

    //* data copy only Stack data type
    //* klo Heap data type maka yg terjadi Ownership movement

    let name1 = String::from("Hello World");
    let name2 = name1; // its move from name1 to name2, and name1 will be removed, name1 unaccessable
    println!("{}", name2);

    let name3 = name2.clone(); // unless you want to copy so the data is newly reconstructed
    println!("{}", name3);
}
