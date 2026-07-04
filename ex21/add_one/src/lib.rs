/// Add one to the given number.
///
/// # Examples
///
/// ```
/// assert_eq!(10, add_one::add_one(9));
/// ```
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_works() {
        assert_eq!(3, add_one(2));
    }
}
