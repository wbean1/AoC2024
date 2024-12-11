advent_of_code::solution!(11);
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref BLINK_CACHE: Mutex<HashMap<(String, usize), usize>> = Mutex::new(HashMap::new());
}

fn blink_many(stone: &str, times: usize) -> usize {
    if times == 0 {
        return 1;
    }

    let answer;

    // Try to get from cache first
    {
        let cache = BLINK_CACHE.lock().unwrap();
        if let Some(cached) = cache.get(&(stone.to_string(), times)) {
            return *cached;
        }
    }

    if stone == "0" {
        answer = blink_many("1", times - 1);
    } else if stone.len() % 2 == 0 {
        let half_digits = stone.len() / 2;
        let divisor = 10_u64.pow(half_digits as u32);

        let stone_num = stone.parse::<u64>().unwrap();
        let right = stone_num % divisor;
        let left = stone_num / divisor;

        answer =
            blink_many(&left.to_string(), times - 1) + blink_many(&right.to_string(), times - 1);
    } else {
        answer = blink_many(
            &(stone.parse::<u64>().unwrap() * 2024).to_string(),
            times - 1,
        );
    }

    {
        let mut cache = BLINK_CACHE.lock().unwrap();
        cache.insert((stone.to_string(), times), answer);
    }

    answer
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones: Vec<&str> = input.split_whitespace().collect();
    Some(
        stones
            .iter()
            .map(|stone| blink_many(stone, 25))
            .sum::<usize>() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones: Vec<&str> = input.split_whitespace().collect();
    Some(
        stones
            .iter()
            .map(|stone| blink_many(stone, 75))
            .sum::<usize>() as u64,
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
