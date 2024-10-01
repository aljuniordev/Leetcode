use std::collections::HashMap;

#[allow(dead_code)]
pub fn solution_force_brute(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if i != j && nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![0, 0]
}

#[allow(dead_code)]
pub fn solution_better_performance(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map:HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        match map.get(&(target - nums[i])) {
            Some(&index) => {
                return vec![index, i as i32]
            },
            None => map.insert(nums[i], i as i32),
        };
    }
    return vec![0, 0];
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_example1_force_brute() {
        assert_eq!(solution_force_brute(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_example1_better_performance() {
        assert_eq!(solution_better_performance(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn test_example2_force_brute() {
        assert_eq!(solution_force_brute(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_example2_better_performance() {
        assert_eq!(solution_better_performance(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn test_example3_force_brute() {
        assert_eq!(solution_force_brute(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_example3_better_performance() {
        assert_eq!(solution_better_performance(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_no_solution_force_brute() {
        assert_eq!(solution_force_brute(vec![1, 2, 3], 7), vec![0, 0]);
    }

    #[test]
    fn test_no_solution_better_performance() {
        assert_eq!(solution_better_performance(vec![1, 2, 3], 7), vec![0, 0]);
    }
}
