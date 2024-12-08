advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum: u32 = 0;
    for line in input.lines() {
        for cap in re.captures_iter(line) {
            let num1: u32 = cap[1].parse().unwrap();
            let num2: u32 = cap[2].parse().unwrap();
            sum += num1 * num2;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut sum: u32 = 0;
    let mut enabled: bool = true;

    for line in input.lines() {
        for cap in re.captures_iter(line) {
            if let Some(_nums) = cap.get(1) {
                if enabled {
                    let num1: u32 = cap[1].parse().unwrap();
                    let num2: u32 = cap[2].parse().unwrap();
                    sum += num1 * num2;
                }
            } else if cap.get(0).unwrap().as_str() == "do()" {
                enabled = true;
            } else if cap.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
