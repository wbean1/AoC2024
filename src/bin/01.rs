advent_of_code::solution!(1);

fn parse_and_sort(input: &str) -> (Vec<u32>, Vec<u32>) {
    let nums: Vec<(u32, u32)> = input
        .trim()
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line
                .split("   ")
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            (numbers[0], numbers[1])
        })
        .collect();
    let (mut v1, mut v2): (Vec<u32>, Vec<u32>) = nums.into_iter().unzip();
    v1.sort();
    v2.sort();
    (v1, v2)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (array_one, array_two) = parse_and_sort(input);
    let mut sum: u32 = 0;
    for i in 0..array_one.len() {
        if array_one[i] > array_two[i] {
            sum += array_one[i] - array_two[i];
        } else {
            sum += array_two[i] - array_one[i];
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (array_one, array_two) = parse_and_sort(input);
    let sum = array_one
        .iter()
        .flat_map(|val| array_two.iter().take_while(|&&x| x <= *val).filter(|&&x| x == *val))
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_sort() {
        let input = "5   2\n1   8\n3   4";
        let (array_one, array_two) = parse_and_sort(input);
        assert_eq!(array_one, vec![1, 3, 5]);
        assert_eq!(array_two, vec![2, 4, 8]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
