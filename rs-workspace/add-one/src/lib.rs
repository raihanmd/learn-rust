#[allow(unused_imports)]
use rand;

/// Adds one to the number given.
///
/// ## Example
///
/// ```
/// use add_one::add_one;
/// let five: i32 = add_one(4);
/// assert_eq!(five, 5);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
