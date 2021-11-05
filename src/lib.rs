//! Roman numerals converter
//!
//! # Quick Start
//!
//! ```
//! use ramenon::*;
//!
//! let int = 3_888;
//! let roman = "MMMDCCCLXXXVIII";
//!
//! let roman_to_int = to_int(roman);
//! let int_to_roman = to_roman(int);
//!
//! assert_eq!(int, roman_to_int.unwrap());
//! assert_eq!(roman, int_to_roman.unwrap().as_str());
//! ```

const THOUSANDS: [(&str, usize); 3] = [("MMM", 3000), ("MM", 2000), ("M", 1000)];

const HUNDREDS: [(&str, usize); 9] = [
    ("CM", 900),
    ("DCCC", 800),
    ("DCC", 700),
    ("DC", 600),
    ("D", 500),
    ("CD", 400),
    ("CCC", 300),
    ("CC", 200),
    ("C", 100),
];

const TENS: [(&str, usize); 9] = [
    ("XC", 90),
    ("LXXX", 80),
    ("LXX", 70),
    ("LX", 60),
    ("L", 50),
    ("XL", 40),
    ("XXX", 30),
    ("XX", 20),
    ("X", 10),
];

const UNITS: [(&str, usize); 9] = [
    ("IX", 9),
    ("VIII", 8),
    ("VII", 7),
    ("VI", 6),
    ("V", 5),
    ("IV", 4),
    ("III", 3),
    ("II", 2),
    ("I", 1),
];

/// The largest value that can be represented by roman numerals.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// assert_eq!(ramenon::MAX, 3_999);
/// ```
pub const MAX: usize = 3_999;

/// Converts a roman numeral to an integer, returning `None` if the provided roman numeral is invalid.
///
/// The following table displays how Roman Numerals are usually written:
///
/// |     | Thousands | Hundreds | Tens | Units |
/// | :-: | :-------: | :------: | :--: | :---: |
/// |  1  |     M     |    C     |  X   |   I   |
/// |  2  |    MM     |    CC    |  XX  |  II   |
/// |  3  |    MMM    |   CCC    | XXX  |  III  |
/// |  4  |           |    CD    |  XL  |  IV   |
/// |  5  |           |    D     |  L   |   V   |
/// |  6  |           |    DC    |  LX  |  VI   |
/// |  7  |           |   DCC    | LXX  |  VII  |
/// |  8  |           |   DCCC   | LXXX | VIII  |
/// |  9  |           |    CM    |  XC  |  IX   |
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let int = ramenon::to_int("MMMCMXCIX");
/// assert_eq!(int, Some(3_999));
///
/// let int = ramenon::to_int("IIV");
/// assert_eq!(int, None);
/// ```
pub fn to_int(roman: &str) -> Option<usize> {
    let mut slice: &str = roman;
    let mut int: usize = 0;
    for token in THOUSANDS {
        if slice.is_empty() {
            break;
        }
        match slice.strip_prefix(token.0) {
            Some(postfix) => {
                slice = postfix;
                int += token.1;
                break;
            }
            None => {
                continue;
            }
        }
    }
    for token in HUNDREDS {
        if slice.is_empty() {
            break;
        }
        match slice.strip_prefix(token.0) {
            Some(postfix) => {
                slice = postfix;
                int += token.1;
                break;
            }
            None => {
                continue;
            }
        }
    }
    for token in TENS {
        if slice.is_empty() {
            break;
        }
        match slice.strip_prefix(token.0) {
            Some(postfix) => {
                slice = postfix;
                int += token.1;
                break;
            }
            None => {
                continue;
            }
        }
    }
    for token in UNITS {
        if slice.is_empty() {
            break;
        }
        match slice.strip_prefix(token.0) {
            Some(postfix) => {
                slice = postfix;
                int += token.1;
                break;
            }
            None => {
                continue;
            }
        }
    }
    if !slice.is_empty() || int == 0 {
        None
    } else {
        Some(int)
    }
}

/// Converts an integer to a roman numeral, returning `None` if the provided integer is invalid.
///
/// Note that interger must be between 1 and 3999.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let roman = ramenon::to_roman(3_888);
/// assert_eq!(roman, Some(String::from("MMMDCCCLXXXVIII")));
///
/// let roman = ramenon::to_roman(4_888);
/// assert_eq!(roman, None);
/// ```
pub fn to_roman(int: usize) -> Option<String> {
    let mut partial: usize = int;
    let mut roman = String::new();
    for token in THOUSANDS {
        if partial == 0 {
            break;
        }
        match partial.checked_sub(token.1) {
            Some(result) => {
                roman.push_str(token.0);
                partial = result;
                break;
            }
            None => continue,
        }
    }
    for token in HUNDREDS {
        if partial == 0 {
            break;
        }
        match partial.checked_sub(token.1) {
            Some(result) => {
                roman.push_str(token.0);
                partial = result;
                break;
            }
            None => continue,
        }
    }
    for token in TENS {
        if partial == 0 {
            break;
        }
        match partial.checked_sub(token.1) {
            Some(result) => {
                roman.push_str(token.0);
                partial = result;
                break;
            }
            None => continue,
        }
    }
    for token in UNITS {
        if partial == 0 {
            break;
        }
        match partial.checked_sub(token.1) {
            Some(result) => {
                roman.push_str(token.0);
                partial = result;
                break;
            }
            None => continue,
        }
    }
    if partial != 0 || roman.is_empty() {
        None
    } else {
        Some(roman)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_3_999_to_roman() {
        let int = 3_999;
        let roman = to_roman(int);
        assert_eq!(roman, Some(String::from("MMMCMXCIX")));
    }

    #[test]
    fn int_3_888_to_roman() {
        let int = 3_888;
        let roman = to_roman(int);
        assert_eq!(roman, Some(String::from("MMMDCCCLXXXVIII")));
    }

    #[test]
    fn error_int_to_roman() {
        let int = 4_888;
        let roman = to_roman(int);
        assert_eq!(roman, None);
    }

    #[test]
    fn error_int_to_roman_again() {
        let int = 0;
        let roman = to_roman(int);
        assert_eq!(roman, None);
    }

    #[test]
    fn roman_3_999_to_int() {
        let roman = "MMMCMXCIX";
        let int = to_int(roman);
        assert_eq!(int, Some(3_999));
    }

    #[test]
    fn roman_3_888_to_int() {
        let roman = "MMMDCCCLXXXVIII";
        let int = to_int(roman);
        assert_eq!(int, Some(3_888));
    }

    #[test]
    fn error_roman_to_int() {
        let roman = "XXXXXXMMMDCCCLXXXVIII";
        let int = to_int(roman);
        assert_eq!(int, None);
    }

    #[test]
    fn error_roman_to_int_again() {
        let roman = "VIIII";
        let int = to_int(roman);
        assert_eq!(int, None);
    }

    #[test]
    fn error_roman_to_int_retry() {
        let roman = "IIV";
        let int = to_int(roman);
        assert_eq!(int, None);
    }
}
