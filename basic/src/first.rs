use crate::second::say_hello as say_hello_two;

pub fn say_hello(name: String) {
	println!("Hello from first {}", name);
	say_hello_two(name);
	foo::bar::baz();
}

pub fn hello_world() {
	println!("Hello World");
}

pub mod foo {
	pub mod bar {
		pub fn baz() {
			super::super::hello_world();
		}
	}
}

// This anotation for specifier to compiler that to compile this when cargo test
#[cfg(test)]
mod tests {
	#[test]
	fn test_say_hello() {
		let name = String::from("Alice");
		super::say_hello(name);
	}

	#[test]
	fn test_hello_world() {
		super::hello_world();
	}
}
