advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n");
    let mut array_one: Vec<u32> = Vec::new();
    let mut array_two: Vec<u32> = Vec::new();
    for line in lines {
        if line == "" { continue }
        let split = line.split("   ").collect::<Vec<&str>>();
        array_one.push(split[0].parse().unwrap());
        array_two.push(split[1].parse().unwrap());
    }
    array_one.sort();
    array_two.sort();
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
    let lines = input.split("\n");
    let mut array_one: Vec<u32> = Vec::new();
    let mut array_two: Vec<u32> = Vec::new();
    for line in lines {
        if line == "" { continue }
        let split = line.split("   ").collect::<Vec<&str>>();
        array_one.push(split[0].parse().unwrap());
        array_two.push(split[1].parse().unwrap());
    }
    array_one.sort();
    array_two.sort();
    let mut sum: u32 = 0;
    for i in 0..array_one.len() {
        for j in 0..array_two.len() {
            if array_one[i] == array_two[j] {
                sum += array_one[i];
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
