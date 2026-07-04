/// Add two to the given number.
///
/// # Examples
///
/// ```
/// assert_eq!(11, add_two::add_two(9));
/// ```
///
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_works() {
        assert_eq!(4, add_two(2));
    }
}
