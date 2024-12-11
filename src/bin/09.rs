advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let original = parse_input(input);
    let id_vector = convert_to_ids(&original);
    let compacted_vector = compact(&id_vector);
    let checksum = checksum(&compacted_vector);
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let original = parse_input(input);
    let id_vector = convert_to_ids(&original);
    let compacted_vector = compact_part2(&id_vector, id_vector.len());
    let checksum = checksum(&compacted_vector);
    Some(checksum)
}

fn parse_input(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect()
}

fn convert_to_ids(input: &Vec<u8>) -> Vec<i32> {
    let mut id = 0;
    let mut new_vec = Vec::<i32>::new();
    let mut is_id = true;
    for num in input {
        for _ in 0..*num {
            if is_id {
                new_vec.push(id);
            } else {
                new_vec.push(-1);
            }
        }
        if is_id {
            id += 1;
        }
        is_id = !is_id;
    }

    new_vec
}

fn compact(input: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = input.clone();
    let left_most_empty = left_most_empty(&input);
    let right_most_non_empty = right_most_non_empty(&input);
    if left_most_empty < right_most_non_empty {
        new_vec.swap(left_most_empty, right_most_non_empty);
    } else {
        return new_vec;
    }
    return compact(&new_vec);
}

fn compact_part2(input: &Vec<i32>, position: usize) -> Vec<i32> {
    let mut new_vec = input.clone();
    let (origin_pos, len) = find_rightmost_id(&input, position);
    let destination_pos = find_leftmost_empty_space_of_len(&input, len);
    if destination_pos < origin_pos {
        for i in 0..len {
            new_vec.swap(origin_pos + i, destination_pos + i);
        }
    }
    if origin_pos < 10 {
        return new_vec;
    }
    return compact_part2(&new_vec, origin_pos);
}

fn find_rightmost_id(input: &Vec<i32>, position: usize) -> (usize, usize) {
    let mut id = -1;
    let mut ending_pos = 0;
    let mut starting_pos = 0;
    for (i, &val) in input[..position].iter().enumerate().rev() {
        if val != -1 {
            id = val;
            ending_pos = i;
            break;
        }
    }
    for (i, &val) in input[..position].iter().enumerate() {
        if val == id {
            starting_pos = i;
            break;
        }
    }

    (starting_pos, ending_pos - starting_pos + 1)
}

fn find_leftmost_empty_space_of_len(input: &Vec<i32>, len: usize) -> usize {
    let mut count = 0;
    for (i, &val) in input.iter().enumerate() {
        if val == -1 {
            count += 1;
            if count == len {
                return i - len + 1;
            }
        } else {
            count = 0;
        }
    }
    input.len()
}

fn left_most_empty(input: &Vec<i32>) -> usize {
    for (i, &val) in input.iter().enumerate() {
        if val == -1 {
            return i;
        }
    }
    input.len()
}

fn right_most_non_empty(input: &Vec<i32>) -> usize {
    for (i, &val) in input.iter().enumerate().rev() {
        if val != -1 {
            return i;
        }
    }
    0
}

fn checksum(input: &Vec<i32>) -> u64 {
    let mut sum = 0;
    for (i, &val) in input.iter().enumerate() {
        if val == -1 {
            continue;
        } else {
            sum += i as u64 * val as u64;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_rightmost_id() {
        let vec = vec![-1, -1, -1, -1, 3, 3, 3, -1, -1, 4, 4, -1];
        let result = find_rightmost_id(&vec, 7);
        assert_eq!(result, (4, 3));
    }

    #[test]
    fn test_find_rightmost_id_2() {
        let vec = vec![-1, -1, -1, -1, 3, 3, 3, -1, -1, 4, 4, -1];
        let result = find_rightmost_id(&vec, vec.len() - 1);
        assert_eq!(result, (9, 2));
    }

    #[test]
    fn test_find_leftmost_empty_space_of_len() {
        let vec = vec![0, 1, 2, 3, 4, 5, -1, -1, 6, 7, 8, -1, -1, -1, 9, 10];
        let result = find_leftmost_empty_space_of_len(&vec, 3);
        assert_eq!(result, 11)
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
