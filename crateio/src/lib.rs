//! # Crateio
//!
//! `crateio` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given :)
/// 
/// # Examples:
/// 
/// ```
/// let arg = 5;
/// let answer = crateio::add_one(arg);
/// 
/// assert_eq!(answer, 6);
/// ```
/// ```
/// let arg = 10;
/// let answer = crateio::add_one(arg);
/// 
/// assert_eq!(answer, 11);
/// ```
/// 
/// # Errors:
/// 
/// 
pub fn add_one(value: i32) -> i32 {
    value + 1
}

/// Subtracts one of the number given :(
/// 
/// # Examples
/// 
/// ```
/// let arg = 9;
/// let answer = crateio::minus_one(arg); 
/// 
/// assert_eq!(answer, 8);
/// ```
/// ```
/// let arg = 78;
/// let answer = crateio::minus_one(arg); 
/// 
/// assert_eq!(answer, 77);
/// ```
pub fn minus_one(value: i32) -> i32 {
    value - 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
