#[allow(dead_code)]
pub fn solution_1(x: i32) -> bool {
  let num_str: String = x.to_string();
  
  let digits: Vec<char> = num_str.chars().collect();

  let mut reversed_digits: Vec<char> = digits.clone();
  reversed_digits.reverse();

  return digits == reversed_digits;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(solution_1(121), true);
    }

    #[test]
    fn test_example2() {
        assert_eq!(solution_1(-121), false);
    }

    #[test]
    fn test_example3() {
        assert_eq!(solution_1(10), false);
    }
}
