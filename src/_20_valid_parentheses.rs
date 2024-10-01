#[allow(dead_code)]
pub fn solution_1(s: &str) -> bool {
    let mut open_symbols = Vec::new();
    
    for letter in s.chars() {
        match letter {
            '(' | '{' | '[' => open_symbols.push(letter),
            ')' => if open_symbols.pop() != Some('(') { return false; },
            '}' => if open_symbols.pop() != Some('{') { return false; },
            ']' => if open_symbols.pop() != Some('[') { return false; },
            _ => return false, 
        }   
    }

    return open_symbols.is_empty();
    // let closes_symbols = s.chars()
    //                     .into_iter()
    //                     .map(|item| {
    //                         if item == '(' { return Some(')'); }
    //                         if item == '{' { return Some('}'); }
    //                         if item == '[' { return Some(']'); }
    //                         return None;
    //                     });
    
    // return closes_symbols.into_iter().all(|item| {
    //     if item.is_some() { return s.contains(item.unwrap());}
    //     return true;
    // });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(solution_1("()"), true);
    }
    #[test]
    fn test_example2() {
        assert_eq!(solution_1("()[]{}"), true);
    }
    #[test]
    fn test_example3() {
        assert_eq!(solution_1("(]"), false);
    }
    #[test]
    fn test_example4() {
        assert_eq!(solution_1("([])"), true);
    }
    #[test]
    fn test_example5() {
        assert_eq!(solution_1("([)]"), false);
    }
}
