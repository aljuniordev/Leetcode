#[allow(dead_code)]
pub fn solution_1(word: &str, ch: char) -> String {
   if word == "" || ch == ' ' {
    return "".to_string();
   }
   let first_occurrence = word.chars()
                                    .position(|c| c == ch)
                                    .unwrap_or(usize::MAX);
   if first_occurrence == usize::MAX {
    return word.to_string();
   }
   let (prefix, suffix) = word.split_at(first_occurrence+1);

   return format!("{}{}", prefix.chars().rev().collect::<String>(), suffix);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(solution_1("abcdefd", 'd'), "dcbaefd");
    }
    #[test]
    fn test_example2() {
        assert_eq!(solution_1("xyxzxe", 'z'), "zxyxxe");
    }
    #[test]
    fn test_example3() {
        assert_eq!(solution_1("abcd", 'z'), "abcd");
    }
}
