// https://doc.rust-lang.org/std/primitive.u16.html#method.checked_sub

const TABLE: [(&str, usize, usize); 30] = [
    ("MMM", 3000, 2),
    ("MM", 2000, 1),
    ("M", 1000, 0),
    ("CM", 900, 8),
    ("DCCC", 800, 7),
    ("DCC", 700, 6),
    ("DC", 600, 5),
    ("D", 500, 4),
    ("CD", 400, 3),
    ("CCC", 300, 2),
    ("CC", 200, 1),
    ("C", 100, 0),
    ("XC", 90, 8),
    ("LXXX", 80, 7),
    ("LXX", 70, 6),
    ("LX", 60, 5),
    ("L", 50, 4),
    ("XL", 40, 3),
    ("XXX", 30, 2),
    ("XX", 20, 1),
    ("X", 10, 0),
    ("IX", 9, 8),
    ("VIII", 8, 7),
    ("VII", 7, 6),
    ("VI", 6, 5),
    ("V", 5, 4),
    ("IV", 4, 3),
    ("III", 3, 2),
    ("II", 2, 1),
    ("I", 1, 0),
];

pub fn to_int(roman: &str) -> usize {
    let mut skip: usize = 0;
    let mut int: usize = 0;
    for token in TABLE {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        match roman.find(token.0) {
            Some(_) => {
                int += token.1;
                skip = token.2;
            }
            None => continue,
        }
    }
    int
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_3_999_to_int() {
        let roman = "MMMCMXCIX";
        let int = to_int(roman);
        assert_eq!(int, 3_999);
    }

    #[test]
    fn roman_3_888_to_int() {
        let roman = "MMMDCCCLXXXVIII";
        let int = to_int(roman);
        assert_eq!(int, 3_888);
    }
}
