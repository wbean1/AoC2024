advent_of_code::solution!(2);

fn is_safe(line: &[u32]) -> bool {
    if all_increasing(line) || all_decreasing(line) {
        if diff_at_most_three(line) {
            return true;
        }
    }
    false
}

fn is_safe_with_dampener(line: &[u32]) -> bool {
    for i in 0..line.len() {
        let mut new_line = line.to_vec();
        new_line.remove(i);
        if is_safe(&new_line) {
            return true;
        }
    }
    false
}

fn all_increasing(line: &[u32]) -> bool {
    line.windows(2).all(|w| w[0] < w[1])
}

fn all_decreasing(line: &[u32]) -> bool {
    line.windows(2).all(|w| w[0] > w[1])
}

fn diff_at_most_three(line: &[u32]) -> bool {
    line.windows(2)
        .all(|w| (w[1].max(w[0]) - w[1].min(w[0])) <= 3)
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let nums: Vec<u32> = line
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            nums
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let nums = parse_input(input);
    let mut count = 0;
    for line in &nums {
        if is_safe(line) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let nums = parse_input(input);
    let mut count = 0;
    for line in &nums {
        if is_safe_with_dampener(line) {
            count += 1;
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_parse_input() {
        let input = "1 2 3\n4 5 6";
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_all_increasing() {
        assert!(all_increasing(&vec![1, 2, 3, 4]));
        assert!(!all_increasing(&vec![1, 3, 2, 4]));
        assert!(!all_increasing(&vec![4, 3, 2, 1]));
    }

    #[test]
    fn test_all_decreasing() {
        assert!(all_decreasing(&vec![4, 3, 2, 1]));
        assert!(!all_decreasing(&vec![4, 2, 3, 1]));
        assert!(!all_decreasing(&vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_diff_at_most_three() {
        assert!(diff_at_most_three(&vec![1, 2, 4, 6]));
        assert!(diff_at_most_three(&vec![6, 4, 2, 1]));
        assert!(!diff_at_most_three(&vec![1, 5, 2, 6]));
    }
}
