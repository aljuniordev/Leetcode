use std::collections::HashMap;

#[allow(dead_code)]
pub fn solution_1(s: &str) -> i32 {
    let roman_numbers: Vec<(&str, i32)> = vec![
        ("DM", 500), ("CM", 900), ("LM", 950), ("XM", 990), ("VM", 995), ("IM", 999), ("M", 1000), 
        ("CD", 400), ("LD", 450), ("XD", 490), ("VD", 495), ("ID", 499), ("D", 500), 
        ("LC", 50), ("XC", 90), ("VC", 95), ("IC", 99), ("C", 100), 
        ("XL", 40), ("VL", 45), ("IL", 49), ("L", 50), 
        ("VX", 5), ("IX", 9), ("X", 10), 
        ("IV", 4), ("V", 5), ("I", 1)];

    let mut idx: usize = 0;
    let mut total: i32 = 0;
    while idx < s.len() {
        let rest_text: &str = &s[idx..];
        let found: Option<&(&str, i32)> = roman_numbers.iter()
            .find(|(roman, _)| rest_text.starts_with(roman));

        match found {
            Some(&(roman, val)) => {
                total += val;
                idx += roman.len();
            },
            None => {
                idx += 1;
            }
        };
    }
    total
}

#[allow(dead_code)]
pub fn solution_2(s: &str) -> i32 {
    let roman_numbers: HashMap<&str, i32> = vec![
        ("DM", 500), ("CM", 900), ("LM", 950), ("XM", 990), ("VM", 995), ("IM", 999), ("M", 1000), 
        ("CD", 400), ("LD", 450), ("XD", 490), ("VD", 495), ("ID", 499), ("D", 500), 
        ("LC", 50), ("XC", 90), ("VC", 95), ("IC", 99), ("C", 100), 
        ("XL", 40), ("VL", 45), ("IL", 49), ("L", 50), 
        ("VX", 5), ("IX", 9), ("X", 10), 
        ("IV", 4), ("V", 5), ("I", 1)
    ].into_iter().collect();

    let mut idx: usize = 0;
    let mut total = 0;
    let s_len = s.len();
    let roman_numbers_keys = ["DM", "CM", "LM", "XM", "VM", "IM", "M",
        "CD", "LD", "XD", "VD", "ID", "D", "LC", "XC", "VC", "IC", "C",
        "XL", "VL", "IL", "L", "VX", "IX", "X", "IV", "V", "I"
    ];

    while idx < s_len {
        for &key in roman_numbers_keys.iter() {
            if s[idx..].starts_with(key) {
                total += roman_numbers[key];
                idx += key.len();
                break;
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(solution_1("III"), 3);
        assert_eq!(solution_2("III"), 3);
    }

    #[test]
    fn test_example2() {
        assert_eq!(solution_1("LVIII"), 58);
        assert_eq!(solution_2("LVIII"), 58);
    }

    #[test]
    fn test_example3() {
        assert_eq!(solution_1("MCMXCIV"), 1994);
        assert_eq!(solution_2("MCMXCIV"), 1994);
    }
}
