#[allow(dead_code)]
pub fn solution_1(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut first_word = strs[0].to_string();

    for item in strs.iter() {
        if item == &first_word {continue;}

        while !item.starts_with(&first_word) {
            first_word.pop();
            
            if first_word.is_empty() {
                return "".to_string();
            }
        }
    }

    first_word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(solution_1(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl");
    }

    #[test]
    fn test_example2() {
        assert_eq!(solution_1(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "");
    }

    #[test]
    fn test_example3() {
        assert_eq!(solution_1(vec!["ri".to_string(), "ca".to_string()]), "");
    }

    #[test]
    fn test_example4() {
        assert_eq!(solution_1(vec!["cir".to_string(), "car".to_string()]), "c");
    }

    #[test]
    fn test_example5() {
        assert_eq!(solution_1(vec!["aaa".to_string(), "aa".to_string(), "aaa".to_string()]), "aa");
    }
}
