//! # ex20
//!
//! `ex20` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = ex20::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Adds two to the number given
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = ex20::add_two(arg);
///
/// assert_eq!(7, answer);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        let arg = -1;
        let answer = add_one(arg);

        assert_eq!(0, answer);
    }
}

// re-exports to make them available to users of the library
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug, PartialEq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug, PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    /// # Examples
    ///
    /// ```
    /// let primary_color1 = ex20::kinds::PrimaryColor::Red;
    /// let primary_color2 = ex20::kinds::PrimaryColor::Yellow;
    /// let answer = ex20::utils::mix(primary_color1, primary_color2);
    ///
    /// assert_eq!(ex20::kinds::SecondaryColor::Orange, answer);
    /// ```
    ///
    /// using the re-exported names:
    /// ```
    /// let primary_color3 = ex20::PrimaryColor::Red;
    /// let primary_color4 = ex20::PrimaryColor::Blue;
    /// let answer = ex20::mix(primary_color3, primary_color4);
    ///
    /// assert_eq!(ex20::SecondaryColor::Purple, answer);
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Yellow) => SecondaryColor::Orange,
            (PrimaryColor::Red, PrimaryColor::Blue) => SecondaryColor::Purple,
            (PrimaryColor::Yellow, PrimaryColor::Red) => SecondaryColor::Orange,
            (PrimaryColor::Yellow, PrimaryColor::Blue) => SecondaryColor::Green,
            (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
            (PrimaryColor::Blue, PrimaryColor::Yellow) => SecondaryColor::Green,
            _ => panic!("Invalid color combination"),
        }
    }
}
