// https://doc.rust-lang.org/std/primitive.u16.html#method.checked_sub

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

pub fn to_int(roman: &str) -> Result<usize, &'static str> {
    let mut strip: &str = roman;
    let mut int: usize = 0;
    for token in THOUSANDS {
        match strip.strip_prefix(token.0) {
            Some(new_strip) => {
                strip = new_strip;
                int += token.1;
                break;
            }
            None => {
                continue;
            }
        }
    }
    for token in HUNDREDS {
        match strip.strip_prefix(token.0) {
            Some(new_strip) => {
                strip = new_strip;
                int += token.1;
                break;
            }
            None => {
                continue;
            }
        }
    }
    for token in TENS {
        match strip.strip_prefix(token.0) {
            Some(new_strip) => {
                strip = new_strip;
                int += token.1;
                break;
            }
            None => {
                continue;
            }
        }
    }
    for token in UNITS {
        match strip.strip_prefix(token.0) {
            Some(new_strip) => {
                strip = new_strip;
                int += token.1;
                break;
            }
            None => {
                continue;
            }
        }
    }
    if strip.len() > 0 || int == 0 {
        Err("Invalid input")
    } else {
        Ok(int)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_3_999_to_int() {
        let roman = "MMMCMXCIX";
        let int = to_int(roman);
        assert_eq!(int, Ok(3_999));
    }

    #[test]
    fn roman_3_888_to_int() {
        let roman = "MMMDCCCLXXXVIII";
        let int = to_int(roman);
        assert_eq!(int, Ok(3_888));
    }

    #[test]
    fn error_roman_to_int() {
        let roman = "XXXXXXMMMDCCCLXXXVIII";
        let int = to_int(roman);
        assert_eq!(int, Err("Invalid input"));
    }

    #[test]
    fn error_roman_to_int_again() {
        let roman = "VIIII";
        let int = to_int(roman);
        assert_eq!(int, Err("Invalid input"));
    }

    #[test]
    fn error_roman_to_int_retry() {
        let roman = "IIV";
        let int = to_int(roman);
        assert_eq!(int, Err("Invalid input"));
    }
}
