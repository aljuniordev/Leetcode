use std::collections::BinaryHeap;

#[allow(dead_code)]
pub fn solution_1(score: Vec<i32>) -> Vec<String> {
    let mut score_sorted = score.clone();

    score_sorted.sort_unstable_by(|a, b| b.cmp(a));

    return score
        .iter()
        .map(|&item| {
            let ix_rank = score_sorted.iter().position(|&s| s == item).unwrap();
            return match ix_rank {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (ix_rank + 1).to_string(),
            };
        })
        .collect();
}

#[allow(dead_code, unused_variables)]
pub fn solution_2(score: Vec<i32>) -> Vec<String> {
    let mut result: Vec<String> = vec![String::new(); score.len()];
    let mut binary_heap = BinaryHeap::new();
    for (ix, n) in score.into_iter().enumerate() {
        binary_heap.push((n, ix));
    }

    let mut rank = 0;
    while let Some((n, original_ix)) = binary_heap.pop() {
        let res = match rank {
            0 => String::from("Gold Medal"),
            1 => String::from("Silver Medal"),
            2 => String::from("Bronze Medal"),
            _ => (rank + 1).to_string(),
        };

        result[original_ix].push_str(&res);
        rank += 1;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            solution_1(vec![5, 4, 3, 2, 1]),
            vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
        );
    }
    #[test]
    fn test_example2() {
        assert_eq!(
            solution_1(vec![10, 3, 8, 9, 4]),
            vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
        );
    }
}
