// Integration tests are placed in the `tests` directory at the root of your project.
// Each file in `tests/` is a separate test crate. You can use your library crate by importing it.

// Example assuming your library crate is named `belajar`:
use belajar::*; // Replace with your actual crate name

mod common;

#[test]
fn test_hello_world() {
	common::setup();
	// Example test, expand as needed
	assert_eq!("Hello, world!", "Hello, world!");
}

// Add more integration tests below

#[test]
fn test_add_function() {
	// Suppose your library has an `add` function
	assert_eq!(add(2, 3), 5);
	assert_eq!(add(3, 3), 6);
	assert_eq!(add(3, 4), 7);
}

// cargo test --test integration_test <FILE_NAME>
