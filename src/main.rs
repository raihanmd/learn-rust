use std::io;

mod model {
	pub struct User {
		pub name: String,
		pub age: u8,
	}

	impl User {
		pub fn say_hello(&self, name: String) {
			println!("Hello {} my name is {}", name, self.name);
		}
	}
}

mod first;
mod second;

// use first::*;
use first::{hello_world, say_hello};
use second::say_hello as say_hello_two;

fn main() {
	say_hello(String::from("Eko"));
	say_hello_two(String::from("Eko"));
	hello_world();

	let factorial_of = 5;
	let result = factorial(factorial_of);
	println!("Factorial of {} is {}", factorial_of, result);

	println!("Enter a number to calculate its factorial:");
	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read line");

	let num: u32 = input.trim().parse().expect("Please enter a valid number");
	let result = factorial(num);
	println!("Factorial of {} is {}", num, result);

	param_heap(String::from("Hello World"));
	param_stack(40);

	let result_tuple = keep_ownership(String::from("Adit"), String::from("Ya"));
	println!("{} {}", result_tuple.0, result_tuple.1);
	println!("{}", result_tuple.2);

	let _ = full_name(&mut String::from("Adit"), &String::from("Ya"));

	let user_from_module: model::User = model::User {
		name: String::from("Aditya"),
		age: 20,
	};

	user_from_module.say_hello(String::from("Eko"));

	println!("{}", user_from_module.age)
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

	if name2 == name3 {
		println!("same");
	}
}

#[test]
fn if_statement() {
	let num = 9;

	let res = if num > 5 {
		"Greater than 5"
	} else {
		"Less than 5"
	};

	println!("{}", res);
}

#[test]
fn im_looped() {
	let mut count: u8 = 0;
	loop {
		count += 1;
		if count > 10 {
			break;
		} else if count % 2 == 0 {
			continue;
		}

		println!("Count now: {}", count);
	}

	let result = loop {
		count += 1;
		if count == 15 {
			break count * 2;
		}
	};

	println!("Result: {}", result);
}

#[test]
fn looped_in_loop() {
	let mut num = 1;
	'outer: loop {
		let mut i = 1;
		'inner: loop {
			if num > 10 {
				break 'outer;
			}

			println!("{} * {} = {}", num, i, num * i);
			i += 1;
			if i > 10 {
				println!("\n");
				break 'inner;
			}
		}
		num += 1;
	}
}

#[test]
fn while_looped() {
	let mut count = 0;
	while count < 10 {
		count += 1;
		println!("Hello {}", count);
	}
}

#[test]
fn for_looped() {
	let arr = [1, 2, 3, 4, 5];
	for i in arr {
		println!("{}", i);
	}

	let my_range_exclusive = 1..11;
	let my_range_inclusive = 1..=10;

	println!("Start exclusive: {}", my_range_exclusive.start);
	println!("End exclusive: {}", my_range_exclusive.end);
	println!("Start inclusive: {}", my_range_inclusive.start());
	println!("End inclusive: {}", my_range_inclusive.end());

	'outer: for i in my_range_inclusive {
		for j in 1..11 {
			if i % 2 == 0 {
				continue 'outer;
			}
			println!("{} * {} = {}", i, j, i * j);
		}
	}
}

fn factorial(n: u32) -> u32 {
	if n < 1 {
		return 0;
	}

	let mut result = 1;
	for i in 1..=n {
		result *= i;
	}

	result
}

#[test]
fn fn_ownership_param() {
	let int = 10;
	let str = String::from("Hello World");
	param_stack(int);
	let new_str = param_heap(str);
	println!("{}", new_str);

	let first_name = String::from("Aditya");
	let last_name = String::from("Firman");

	let (_, _, full_name) = keep_ownership(first_name, last_name);
	println!("{}", full_name);
}

fn param_stack(int: i32) {
	println!("{}", int);
}

fn param_heap(str: String) -> String {
	println!("{}", str);

	str
}

// * At least you dont want the ownership gone
fn keep_ownership(first_name: String, last_name: String) -> (String, String, String) {
	let full_name = format!("{} {}", first_name, last_name);

	(first_name, last_name, full_name)
}

// * Borrowing is default immutable
#[test]
fn references_borrowing() {
	let mut first_name = String::from("Aditya");
	let last_name = String::from("Firman");

	let full_name = full_name(&mut first_name, &last_name);
	println!("{} from {} and {}", full_name, first_name, last_name);
}

// * In rust cant return reference fron a fn bcz lifetime is over
fn full_name(first_name: &mut String, last_name: &String) -> String {
	first_name.push('!');
	return format!("{} {}", first_name, last_name);
}

#[test]
fn slice() {
	let arr = [1, 2, 3, 4, 5];
	let arr_slice = &arr[1..=3];
	let arr_slice_all = &arr[..];
	println!("{:?}", arr_slice);
	println!("{:?}", arr_slice_all);
}

#[test]
fn str_slice_2() {
	let name = String::from("Eko Kur");
	let slice = &name[..=2];

	println!("{}", slice);
	println!("{}", name);
}

#[test]
fn my_struct() {
	struct House {
		length: u32,
		width: u32,
		height: u32,
		color: String,
	}

	let color = String::from("red");

	let house = House {
		length: 20,
		width: 20,
		height: 3,
		// * Ownership pindah yg color
		color,
	};

	println!("{} {} {}", house.length, house.width, house.height,);
	println!("{}", house.color);

	// * Struct update syntax
	let new_color = String::from("blue");

	// * But bruh if Heap data type, ownership pindah
	let new_house = House {
		color: new_color,
		..house
	};

	// * unless do clone
	let new_house_again = House {
		color: new_house.color.clone(),
		..house
	};

	println!("{}", new_house_again.color);
	println!("{} {}", new_house.color, new_house.height);
}

#[test]
fn tuple_struct() {
	struct Vector2(f32, f32);

	let v = Vector2(10.0, 20.0);
	println!("{} {}", v.0, v.1);
}

#[test]
fn struct_no_field() {
	struct Nothing;

	let _nothing = Nothing;
}

#[test]
fn method() {
	struct House {
		length: u32,
		width: u32,
	}

	// * Method
	impl House {
		fn get_area(&self) -> u32 {
			self.length * self.width
		}
	}

	let house = House {
		length: 20,
		width: 20,
	};

	println!("{}", house.get_area());

	struct Vector2(f32, f32);

	// * Associated function diff in &self on first param
	impl Vector2 {
		fn new(x: f32, y: f32) -> Vector2 {
			Vector2(x, y)
		}
	}

	let v = Vector2::new(10.0, 20.0);
	println!("{} {}", v.0, v.1);
}

#[test]
fn my_enum() {
	enum Direction {
		Up,
		// * Test only unused do underscore
		_Down,
		_Left,
		_Right,
	}

	let direction = Direction::Up;

	match direction {
		Direction::Up => println!("Up"),
		Direction::_Down => println!("Down"),
		Direction::_Left => println!("Left"),
		Direction::_Right => println!("Right"),
	}

	enum Payment {
		#[allow(dead_code)]
		Card(String),
		#[allow(dead_code)]
		BankTransfer(String, String),
	}

	impl Payment {
		fn pay(&self, amount: f64) {
			println!("Paying {}", amount)
		}
	}

	let _payment_card = Payment::Card(String::from("08923721388"));
	let _payment_bank = Payment::BankTransfer(String::from("Bank BCA"), String::from("321"));

	_payment_card.pay(100_000.00);
}

#[test]
fn pattern_matching() {
	enum Direction {
		Up,
		// * Test only unused do underscore
		_Down,
		_Left,
		_Right,
	}

	let direction = Direction::Up;

	match direction {
		Direction::Up => {
			println!("Up")
		}
		Direction::_Down => println!("Down"),
		Direction::_Left => println!("Left"),
		Direction::_Right => println!("Right"),
	}

	enum Payment {
		#[allow(dead_code)]
		Card(String),
		#[allow(dead_code)]
		BankTransfer(String, String),
	}

	impl Payment {
		fn pay(&self, amount: f64) {
			match self {
				Payment::Card(number) => {
					println!("Paying {} with card {}", amount, number)
				}
				Payment::BankTransfer(bank, num) => {
					println!("Paying {} with bank {} {}", amount, bank, num)
				}
			}
		}
	}

	let payment = Payment::Card(String::from("1234-5678-9012-3456"));

	payment.pay(200.000);

	let name = "Aditya";

	match name {
		"Lynx" => {
			println!("Hello Lynx")
		}
		"Aditya" | "Adit" | "Eko" => {
			println!("Hello Keluarga Aditya")
		}
		other => {
			println!("Hello {}", other)
		}
	}

	let benefit = 100_000;

	match benefit {
		0..=10_000 => {
			println!("Low")
		}
		10_001..=50_000 => {
			println!("Medium")
		}
		_ => {
			println!("High")
		}
	}

	struct GeoPoint(f32, f32);

	let point = GeoPoint(0.0, 01.0);

	match point {
		// * Destructuring
		GeoPoint(x, 0.0) => {
			println!("Your point is on x axis: {}", x)
		}
		GeoPoint(_, y) => {
			println!("Your point is on y axis: {}", y)
		}
	}

	struct Person {
		first_name: String,
		last_name: String,
	}

	let person = Person {
		first_name: String::from("Aditya"),
		last_name: String::from("Firman"),
	};

	match person {
		Person {
			first_name,
			last_name,
		} if last_name == "Firmansyah" => {
			println!("Hello Mr. {}", first_name)
		}
		Person { first_name, .. } => {
			println!("Hello Mr. {}", first_name)
		}
	}

	let value = 2;
	let result = match value {
		1 => "one",
		2 => "two",
		3 => "three",
		_ => "other",
	};

	println!("{}", result);
}

#[test]
fn type_alias() {
	type Umur = u8;

	let umur: Umur = 20;

	println!("Umur: {}", umur);
}

#[test]
fn test_use() {
	first::foo::bar::baz();
}

#[allow(dead_code)]
trait Animal {
	fn name(&self) -> String;
	fn noise(&self) -> String;

	// * Default implementation
	fn talk(&self) {
		println!("{} says {}", self.name(), self.noise())
	}
}

#[allow(dead_code)]
struct Dog {
	name: String,
	noise: String,
}

impl Animal for Dog {
	fn name(&self) -> String {
		self.name.clone()
	}
	fn noise(&self) -> String {
		self.noise.clone()
	}
}

#[test]
fn my_trait() {
	let dog = Dog {
		name: String::from("Dog"),
		noise: String::from("Woof"),
	};

	dog.talk();
	dog.noise();

	fn print_talk(animal: &impl Animal) {
		animal.talk();
	}

	print_talk(&dog)
}
