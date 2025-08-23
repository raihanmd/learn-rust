use colored::*;
use rand::Rng;
use std::{
	fmt::Debug,
	io,
	ops::{Add, Deref},
	rc::Rc,
};

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

	println!("{}", user_from_module.age);

	println!("+++++++++++++ Lets play some guessing +++++++++++++");

	let secret = rand::rng().random_range(0..=100);

	println!("Secret: {}", secret);

	println!("Just guess some number between 1 to 100");

	loop {
		println!("Please input your guess: ");

		let mut guess = String::new();
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please input correct type buddy");
				continue;
			}
		};

		match guess.cmp(&secret) {
			std::cmp::Ordering::Equal => {
				println!("{}", "You win!".green());
				break;
			}
			std::cmp::Ordering::Greater => println!("{}", "Too big!".red()),
			std::cmp::Ordering::Less => println!("{}", "Too small!".red()),
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
	// fn unit() -> () {
	// 	println!("unit");
	// }

	// let res = unit();
	// println!("{:?}", res);

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
	format!("{} {}", first_name, last_name)
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
		fn new(x: f32, y: f32) -> Self {
			Self(x, y)
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
trait Have4Legs {
	fn legs(&self) -> u8;
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

impl Have4Legs for Dog {
	fn legs(&self) -> u8 {
		4
	}
}

#[allow(dead_code)]
fn create_animal(name: String, noise: String) -> impl Animal + Have4Legs {
	Dog { name, noise }
}

#[test]
fn my_trait() {
	let dog = Dog {
		name: String::from("Ascher"),
		noise: String::from("Woof"),
	};

	println!("dog name {}", dog.name());

	fn print_talk(animal: &(impl Animal + Have4Legs)) {
		animal.talk();
		println!("{} legs", animal.legs());
	}

	let new_dog = create_animal(String::from("Frasch"), String::from("Meow"));

	print_talk(&dog);
	print_talk(&new_dog);

	// * if have conflict method it can used
	Animal::talk(&new_dog)
}

#[test]
fn super_trait() {
	trait AnimalWith4Legs: Animal + Have4Legs {
		fn walk(&self) {
			// * self.name() from trait animal
			println!("{} is walking with 4 legs", self.name())
		}
	}

	struct Cat {
		name: String,
		noise: String,
	}

	impl Animal for Cat {
		fn name(&self) -> String {
			self.name.clone()
		}
		fn noise(&self) -> String {
			self.noise.clone()
		}
	}

	impl Have4Legs for Cat {
		fn legs(&self) -> u8 {
			4
		}
	}

	impl AnimalWith4Legs for Cat {
		fn walk(&self) {
			// * self.name() from trait animal
			println!("cat {} is walking with 4 legs", self.name())
		}
	}

	fn print_talk(animal: &impl AnimalWith4Legs) {
		animal.talk();
		animal.walk();
	}

	let my_cat: Cat = Cat {
		name: String::from("Garfield"),
		noise: String::from("Meow"),
	};

	print_talk(&my_cat);
}

#[allow(dead_code)]
struct Point<T = i32> {
	x: T,
	y: T,
}

impl<T> Point<T> {
	#[allow(dead_code)]
	fn get_x(&self) -> &T {
		&self.x
	}
	#[allow(dead_code)]
	fn get_y(&self) -> &T {
		&self.y
	}
}

#[allow(dead_code)]
enum Value<T> {
	None,
	Value(T),
}

#[test]
fn generic() {
	trait CanSayHello {
		fn say_hello(&self);
	}

	#[allow(dead_code)]
	trait GetValue<T>
	where
		T: PartialOrd,
	{
		fn get_value(&self) -> &T;
	}

	impl CanSayHello for Point<i32> {
		fn say_hello(&self) {
			println!("Hello");
		}
	}

	impl<T> GetValue<T> for Point<T>
	where
		T: PartialOrd,
	{
		fn get_value(&self) -> &T {
			&self.x
		}
	}

	let my_point = Point::<i32> { x: 10, y: 20 };
	let float = Point { x: 10.0, y: 20.0 };
	println!("{} {}", my_point.get_x(), my_point.get_y());
	println!("{}", my_point.get_value());

	println!("float {}", float.get_value());

	let my_value = Value::<i32>::Value(100);
	match my_value {
		Value::None => {
			println!("NONE")
		}
		Value::Value(value) => {
			println!("{}", value)
		}
	}

	struct Hi<T: CanSayHello> {
		value: T,
	}

	let hi = Hi::<Point<i32>> {
		value: Point::<i32> { x: 10, y: 20 },
	};

	hi.value.say_hello();
}

#[allow(dead_code)]
fn min<T: PartialOrd>(a: T, b: T) -> T {
	if a < b {
		a
	} else {
		b
	}
}

#[test]
fn test_min() {
	let a = min(10, 20);
	let b = min(9, 10);
	println!("{} {}", a, b);
}

struct Block {
	x: i32,
	y: i32,
	z: i32,
}

impl Add for Block {
	type Output = Block;

	fn add(self, rhs: Self) -> Self::Output {
		Block {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z,
		}
	}
}

impl Debug for Block {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Block Debug")
			.field("x", &self.x)
			.field("y", &self.y)
			// .field("z", &self.z)
			.finish()
	}
}

// !Primitive datas use Display, but for complex one, like arr, slice using Debug
// impl Display for Block {

// }

#[test]
fn test_overradable_operator() {
	let block_1 = Block { x: 1, y: 2, z: 3 };
	let block_2 = Block { x: 4, y: 5, z: 6 };
	let block_3 = block_1 + block_2;

	println!("{:?}", block_3);
	// println!("{}", block_3);
}

// Comparing ops can be implemented too in core::cmp
#[test]
fn optional_value() {
	let x: Option<i32> = None; // * None is optional value

	println!("{:?}", x);

	let double_some = double(Some(10));
	println!("{:?}", double_some);

	let double_none = double(None);
	println!("{:?}", double_none);
}

#[allow(dead_code)]
fn double(x: Option<i32>) -> Option<i32> {
	x.map(|x| x * 2)
}

#[test]
fn test_closure() {
	let sum = |a: i32, b: i32| -> i32 { a + b };

	println!("{}", sum(10, 20));

	fn please_use_closure(val: String, callback: fn(String) -> String) {
		println!("Result {}", callback(val));
	}

	fn to_uppercase(val: String) -> String {
		val.to_uppercase()
	}

	please_use_closure(String::from("Hello World"), to_uppercase);
}

#[allow(dead_code)]
#[derive(Debug)]
enum IpAddress {
	V4(u8, u8, u8, u8),
	V6(String),
}

#[test]
fn test_ipaddr() {
	let ip_v4 = IpAddress::V4(192, 168, 10, 1);
	let ip_v6 = IpAddress::V6(String::from("::1"));

	println!("{:#?}", ip_v4);
	println!("{:#?}", ip_v6);
}

#[test]
fn test_option() {
	let num = Some(30);

	println!("{:?}", num);

	if let Some(30) = num {
		println!("30 buddy!")
	}
}

#[allow(dead_code)]
enum Operation {
	Add(i32, i32),
	Mul(i32, i32),
	Sub { first: i32, second: i32 },
	Div { divident: i32, divisor: i32 },
}

#[allow(dead_code)]
impl Operation {
	fn execute(self) -> Result<i32, String> {
		match self {
			Self::Add(a, b) => Ok(a + b),
			Self::Mul(a, b) => Ok(a * b),
			Self::Sub { first, second } => Ok(first - second),
			Self::Div { divident, divisor } => {
				if divisor == 0 {
					Err(String::from("Can not divide by zero"))
				} else {
					Ok(divident / divisor)
				}
			}
		}
	}
}

#[test]
fn test_result() {
	let user_input = Operation::Div {
		divident: 20,
		divisor: 0,
	};
	match user_input.execute() {
		Result::Ok(res) => println!("Result: {res}"),
		Result::Err(e) => println!("Error: {e}"),
	}
}

#[allow(dead_code)]
fn test_print(param: String) {
	println!("HELLO {param}");
}

#[test]
fn test_vec() {
	let vec = vec!["Boby".to_owned(), "Aditya".to_owned()];

	for item in &vec {
		println!("{}", item);
	}
}

#[test]
fn hash_data_type() {
	use std::collections::HashMap;

	let mut hash_map: HashMap<String, i32> = HashMap::new();

	hash_map.insert("Aditya".to_owned(), 20);

	let get = hash_map.get("Aditya");

	println!("HashMap Aditya: {:?}", hash_map.get("Aditya"));

	if let Some(value) = get {
		println!("Aditya is {}", value);
	} else {
		println!("Aditya not found");
	}
}

#[test]
fn btree_hash() {
	// !Auto diurutkan, klo str dia ascending
	let mut btree = std::collections::BTreeMap::new();
	btree.insert("Aditya".to_owned(), 20);
	btree.insert("Boby".to_owned(), 30);

	println!("BTreeMap Aditya: {:?}", btree.get("Aditya"));
	println!("BTreeMap Boby: {:?}", btree.get("Boby"));

	for (key, value) in &btree {
		println!("{}: {}", key, value);
	}
}

#[test]
fn set_data_type() {
	// !HashSet, BTreeSet
	use std::collections::HashSet;

	let set = HashSet::from(["Aditya", "Boby", "Eko"]);

	for item in &set {
		println!("{item}");
	}
}

#[test]
fn js_like() {
	let nums = [1, 2, 3, 4, 5];

	let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();

	println!("Doubled: {doubled:?}");
}

#[test]
fn test_error() {
	panic!("This is a panic message");
}

#[test]
fn question_mark_use() {
	fn divide(a: i32, b: i32) -> Result<i32, String> {
		if b == 0 {
			Err(String::from("Cannot divide by zero"))
		} else {
			Ok(a / b)
		}
	}

	fn calculate() -> Result<i32, String> {
		let result = divide(10, 2)?;
		let another_result = divide(result, 0)?;
		Ok(another_result)
	}

	println!("{:?}", calculate());
}

#[test]
fn lifetime_anotation() {
	fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
		if s1.len() > s2.len() {
			s1
		} else {
			s2
		}
	}

	let str1 = String::from("Hello");
	let str2 = String::from("World");

	let result = longest(&str1, &str2);
	println!("Longest string is: {}", result);
}

#[test]
fn smart_pointer() {
	let five = Box::new(5); // Box is a smart pointer that allocates memory on the heap
	println!("Value in box: {}", five);

	display_number(*five);
	display_number_ref(&five);

	fn display_number(num: i32) {
		println!("Number in box non ref: {}", num);
	}

	fn display_number_ref(num: &i32) {
		println!("Number in box ref: {}", num);
	}
}

#[allow(dead_code)]
enum Category {
	Of(String, Box<Category>),
	None,
}

#[test]
fn test_recurce_enum() {
	let category = Category::Of(
		String::from("Electronics"),
		Box::new(Category::Of(
			String::from("Computers"),
			Box::new(Category::None),
		)),
	);

	match category {
		Category::Of(name, sub_category) => {
			println!("Category: {}", name);
			if let Category::Of(sub_name, _) = *sub_category {
				println!("Sub-category: {}", sub_name);
			}
		}
		Category::None => println!("No sub-category"),
	}
}

#[test]
fn test_dereference() {
	let a = Box::new(5);

	println!("DOubled: {}", *a * 2)
}

#[allow(dead_code)]
struct MyValue<T> {
	value: T,
}

impl<T> Deref for MyValue<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.value
	}
}

#[test]
fn test_deref() {
	let my_value = MyValue { value: 10 };

	// * Deref coercion
	let x: &i32 = &my_value; // Automatically dereferenced
	println!("Value: {}", x);

	// * Can also use directly
	println!("Value from deref: {}", *my_value);

	introduce(&my_value); // Deref coercion happens here too

	fn introduce(val: &i32) {
		println!("Hello, my value is: {}", val);
	}
}

impl<T> Drop for MyValue<T> {
	fn drop(&mut self) {
		println!("Dropping MyValue");
	}
}

#[test]
fn test_drop() {
	let my_value = MyValue { value: 10 };
	println!("MyValue created with value: {}", my_value.value);
}

#[allow(dead_code)]
enum Brand {
	Of(String, Rc<Brand>),
	None,
}

#[test]
fn multiple_owner() {
	let apple = Rc::new(Brand::Of(String::from("Apple"), Rc::new(Brand::None)));

	let laptop = Rc::new(Brand::Of(
		String::from("Laptop"),
		Rc::clone(&apple), // Cloning the Rc increases the reference count
	));

	println!("Apple: {:?}", Rc::strong_count(&apple));
	println!("Laptop: {:?}", Rc::strong_count(&laptop));
}

#[test]
fn ref_cell() {
	use std::cell::RefCell;

	let value = RefCell::new(5);

	*value.borrow_mut() += 1;

	println!("Value: {}", value.borrow());
}

#[test]
fn ref_cell_test() {
	use std::cell::RefCell;

	#[allow(dead_code)]
	#[derive(Debug)]
	struct Person {
		name: RefCell<String>,
		age: RefCell<i32>,
	}

	let adit = Person {
		name: RefCell::new(String::from("Aditya")),
		age: RefCell::new(20),
	};

	{
		let mut adit_name = adit.name.borrow_mut();
		*adit_name = "Firman".to_string();
	}

	println!("{adit:?}");
}

#[allow(dead_code)]
static mut GLOBAL_VAR: i32 = 10;

#[allow(dead_code)]
unsafe fn increment() {
	GLOBAL_VAR += 1;
}

#[test]
fn static_test() {
	unsafe {
		increment();
		// println!("Global variable: {}", GLOBAL_VAR);
	}
}

#[allow(unused_macros)]
macro_rules! hi {
	() => {
		println!("Hello from macro!");
	};
	($name:expr) => {
		println!("Hello, {}!", $name);
	};
}

#[allow(unused_macros)]
macro_rules! iterate {
	($($item:expr),*) => {
		$(
			println!("{}", $item);
		)*
	};
}

#[test]
fn macro_test() {
	hi!();
	hi!("Aditya");

	hi! {
		"Aditya 2"
	}

	iterate!(1, 2, 3, 4, 5);
}

#[test]
fn file() {
	use std::fs::{self};
	use std::io::{self};

	fn read_user_from_file() -> Result<String, io::Error> {
		// let mut content = String::new();
		// File::open("user.txt")?.read_to_string(&mut content)?;
		fs::read_to_string("user.txt")
	}

	match read_user_from_file() {
		Ok(content) => println!("File content: {}", content),
		Err(e) => println!("Error reading file: {}", e),
	}
}

#[test]
#[ignore]
fn feature() {
	// This test is ignored, you can run it with `cargo test -- --ignored`
	println!("This test is ignored");
}

// run test based on module, first name of func, etc

#[test]
fn docs_closure() {
	#[derive(Debug, PartialEq, Clone, Copy)]
	enum ShirtColor {
		Red,
		Blue,
	}

	struct Inventory {
		shirts: Vec<ShirtColor>,
	}

	impl Inventory {
		fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
			user_preference.unwrap_or_else(|| self.most_stocked())
			// .unwrap_or_else(Vec::new) //! will generate []
		}

		fn most_stocked(&self) -> ShirtColor {
			let mut red: i32 = 0;
			let mut blue: i32 = 0;

			for shirt in &self.shirts {
				match shirt {
					ShirtColor::Red => red += 1,
					ShirtColor::Blue => blue += 1,
				}
			}

			if red > blue {
				ShirtColor::Red
			} else {
				ShirtColor::Blue
			}
		}
	}

	let store = Inventory {
		shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
	};

	let user_pref1 = Some(ShirtColor::Red);
	let giveaway1 = store.giveaway(user_pref1);
	println!(
		"The user with preference {:?} gets {:?}",
		user_pref1, giveaway1
	);

	let user_pref2 = None;
	let giveaway2 = store.giveaway(user_pref2);
	println!(
		"The user with preference {:?} gets {:?}",
		user_pref2, giveaway2
	);

	let mut s = String::from("hello");

	let mut move_closure = || {
		s.push_str(", world");
		println!("{}", s);
		// s //! if only, FnOnce
	};

	// println!("{}", s); //! Error

	move_closure();
}

#[test]
fn some_test() {
	let v1 = vec![2, 4, 6, 2, 3];

	let v2 = v1.iter().map(|x| x * 2).collect::<Vec<_>>();
	// let v3_into_iter = v1.into_iter();

	println!("{:?}", v1);
	println!("{:?}", v2);

	#[derive(PartialEq, Debug)]
	struct Shoe {
		size: u32,
		style: String,
	}

	fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
		shoes.into_iter().filter(|s| s.size == shoe_size).collect()
	}

	let shoes = vec![
		Shoe {
			size: 10,
			style: String::from("sneaker"),
		},
		Shoe {
			size: 13,
			style: String::from("sandal"),
		},
		Shoe {
			size: 10,
			style: String::from("boot"),
		},
	];

	let in_my_size = shoes_in_size(shoes, 10);

	println!("{:?}", in_my_size);

	let items = ["BTC", "ETH", "SOL"];

	for (i, item) in items.iter().enumerate() {
		println!("No {}. {}", i + 1, item);
	}
}

#[test]
fn rc_eg() {
	let (truck_1, truck_2, truck_3) = (Rc::new("Truck 1"), Rc::new("Truck 2"), Rc::new("Truck 3"));

	let shipment_one = vec![truck_1, Rc::clone(&truck_2)];
	let shipment_two = vec![Rc::clone(&truck_2), truck_3];

	println!("{:?}", shipment_one);

	println!("Truck 2 Count: {}", Rc::strong_count(&truck_2));

	drop(shipment_one);

	println!("Truck 2 Count: {}", Rc::strong_count(&truck_2));

	println!("{:?}", shipment_two);
}

#[test]
fn test_ref_cell() {
	trait Messenger {
		fn send(&self, message: &str);
	}

	struct LimitTracker<'a, T: Messenger> {
		messenger: &'a T,
		value: usize,
		limit: usize,
	}

	impl<'a, T> LimitTracker<'a, T>
	where
		T: Messenger,
	{
		pub fn new(messenger: &T, limit: usize) -> LimitTracker<T> {
			LimitTracker {
				messenger,
				value: 0,
				limit,
			}
		}

		pub fn set_value(&mut self, value: usize) {
			self.value = value;
			let percentage = self.value as f64 / self.limit as f64;

			match percentage {
				p if p >= 1.0 => {
					self.messenger.send("Error: Limit reached!");
				}
				p if p >= 0.9 => {
					self.messenger.send("Warning: 90% limit used!");
				}
				p if p >= 0.75 => {
					self.messenger.send("Warning: 75% limit used!");
				}
				_ => {}
			}
		}
	}

	use std::cell::RefCell;

	/// Test section
	struct MockMessenger {
		sent_messages: RefCell<Vec<String>>,
	}

	impl MockMessenger {
		fn new() -> MockMessenger {
			MockMessenger {
				sent_messages: RefCell::new(Vec::new()),
			}
		}
	}

	impl Messenger for MockMessenger {
		fn send(&self, message: &str) {
			self.sent_messages.borrow_mut().push(message.to_string());
		}
	}

	fn it_sends_an_over_75_percent_warning() {
		let messenger = MockMessenger::new();
		let mut tracker = LimitTracker::new(&messenger, 100);

		tracker.set_value(80);

		assert_eq!(messenger.sent_messages.borrow().len(), 1);
		assert_eq!(
			messenger.sent_messages.borrow()[0],
			"Warning: 75% limit used!"
		);
	}

	it_sends_an_over_75_percent_warning();
}

#[test]
fn weak_sp() {
	use std::cell::RefCell;
	use std::rc::Weak;

	#[derive(Debug)]
	struct Node {
		#[allow(dead_code)]
		id: i32,
		parent: RefCell<Weak<Node>>,
		#[allow(dead_code)]
		children: RefCell<Vec<Rc<Node>>>,
	}

	let leaf: Rc<Node> = Rc::new(Node {
		id: 2,
		parent: RefCell::new(Weak::new()),
		children: RefCell::new(Vec::new()),
	});

	println!("Leaf parent: {:?}", leaf.parent.borrow().upgrade());

	let branch: Rc<Node> = Rc::new(Node {
		id: 1,
		parent: RefCell::new(Weak::new()),
		children: RefCell::new(vec![Rc::clone(&leaf)]),
	});

	println!("Branch: {branch:?}");

	*leaf.parent.borrow_mut() = Rc::downgrade(&branch);

	println!(
		"Branch strong count {}, weak count {}",
		Rc::strong_count(&branch),
		Rc::weak_count(&branch)
	);

	println!("Leaf updated: {:?}", leaf.parent.borrow().upgrade());

	println!(
		"Leaf strong count {}, weak count {}",
		Rc::strong_count(&leaf),
		Rc::weak_count(&leaf)
	);
}

#[test]
fn polymorphism() {
	trait Draw {
		fn draw(&self);
	}

	struct Screen {
		components: Vec<Box<dyn Draw>>, // polymorphic
	}

	impl Screen {
		fn run(&self) {
			for component in &self.components {
				component.draw();
			}
		}
	}

	struct Button {
		label: String,
	}

	impl Draw for Button {
		fn draw(&self) {
			println!("Button: {}", self.label);
		}
	}

	struct SelectBox {
		options: Vec<String>,
	}

	impl Draw for SelectBox {
		fn draw(&self) {
			println!("SelectBox: {:?}", self.options);
		}
	}

	let my_screen = Screen {
		components: vec![
			Box::new(SelectBox {
				options: vec![
					String::from("Yes"),
					String::from("No"),
					String::from("Maybe"),
				],
			}),
			Box::new(Button {
				label: String::from("OK"),
			}),
			Box::new(Button {
				label: String::from("Cancel"),
			}),
		],
	};

	my_screen.run();
}

#[test]
fn state_design_pattern() {
	trait State {
		fn request_review(self: Box<Self>) -> Box<dyn State>;
		fn approve(self: Box<Self>) -> Box<dyn State>;
		fn content<'a>(&self, _: &'a Post) -> &'a str {
			""
		}
	}

	struct Post {
		state: Option<Box<dyn State>>,
		content: String,
	}

	impl Post {
		fn new() -> Post {
			Post {
				state: Some(Box::new(Draft)),
				content: String::new(),
			}
		}
	}

	impl Post {
		fn add_text(&mut self, text: &str) {
			self.content.push_str(text);
		}

		fn content(&self) -> &str {
			self.state.as_ref().unwrap().content(self)
		}

		fn request_review(&mut self) {
			if let Some(st) = self.state.take() {
				self.state = Some(st.request_review());
			}
		}

		fn approve(&mut self) {
			if let Some(st) = self.state.take() {
				self.state = Some(st.approve());
			}
		}
	}

	struct Draft;

	impl State for Draft {
		fn request_review(self: Box<Self>) -> Box<dyn State> {
			Box::new(PendingReview)
		}

		fn approve(self: Box<Self>) -> Box<dyn State> {
			self
		}
	}

	struct PendingReview;

	impl State for PendingReview {
		fn request_review(self: Box<Self>) -> Box<dyn State> {
			self
		}

		fn approve(self: Box<Self>) -> Box<dyn State> {
			Box::new(Published)
		}
	}

	struct Published;

	impl State for Published {
		fn request_review(self: Box<Self>) -> Box<dyn State> {
			self
		}

		fn approve(self: Box<Self>) -> Box<dyn State> {
			self
		}

		fn content<'a>(&self, post: &'a Post) -> &'a str {
			&post.content
		}
	}

	let mut post_1 = Post::new();

	post_1.add_text("I ate a salad for lunch today");
	assert_eq!("", post_1.content());

	post_1.request_review();
	assert_eq!("", post_1.content());

	post_1.approve();
	assert_eq!("I ate a salad for lunch today", post_1.content());
}

#[test]
fn another_approach() {
	struct Post {
		content: String,
	}

	struct DraftPost {
		content: String,
	}

	impl Post {
		fn new() -> DraftPost {
			DraftPost {
				content: String::new(),
			}
		}

		fn content(&self) -> &str {
			&self.content
		}
	}

	impl DraftPost {
		fn add_text(&mut self, text: &str) {
			self.content.push_str(text);
		}

		fn request_review(self) -> PendingReviewPost {
			PendingReviewPost {
				content: self.content,
			}
		}
	}

	struct PendingReviewPost {
		content: String,
	}

	impl PendingReviewPost {
		fn approve(self) -> Post {
			Post {
				content: self.content,
			}
		}
	}

	let mut post = Post::new();

	post.add_text("Hello World");

	let post = post.request_review();

	let post = post.approve();

	println!("{}", post.content());
}
