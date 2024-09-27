#[allow(dead_code)]
pub fn solution_1(strs: Vec<String>) -> String {
    let mut parts: Vec<&str> = Vec::new();

    let first_word:&String = &strs[0];

    for i in 0..first_word.len() {
        parts.push(&first_word[i..]);
        parts.push(&first_word[..i+1]);
    }
    parts.iter().for_each(|&i| println!("{}", i));
    let longest_prefix: Vec<&&str> = parts
        .iter()
        .filter(|&part| strs.iter().all(|item| item.contains(part)))
        .collect();

    return match longest_prefix.last() {
        Some(&res) => res.to_string(),
        None => "".to_string( ),
    };
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
        assert_eq!(solution_1(vec!["cir".to_string(), "car".to_string()]), "");
    }
}
