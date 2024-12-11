use std::collections::HashSet;

advent_of_code::solution!(10);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let trailheads = find_trailheads(&map);
    let mut sum = 0;
    for trailhead in trailheads {
        sum += score_trailhead(&map, trailhead);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let trailheads = find_trailheads(&map);
    let mut sum = 0;
    for trailhead in trailheads {
        sum += rate_trailhead(&map, trailhead);
    }
    Some(sum)
}

fn find_trailheads(map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &val) in row.iter().enumerate() {
            if val == 0 {
                trailheads.push((row_idx, col_idx));
            }
        }
    }
    trailheads
}

fn score_trailhead(map: &Vec<Vec<u8>>, trailhead: (usize, usize)) -> u32 {
    let mut visited = HashSet::new();
    visited.insert(trailhead);
    let mut previous_visited_count = 0;
    while visited.len() > previous_visited_count {
        previous_visited_count = visited.len();
        let locations: Vec<_> = visited.iter().cloned().collect();
        for location in locations {
            let value = map[location.0][location.1];
            if location.0 < map.len() - 1 && map[location.0 + 1][location.1] == value + 1 {
                visited.insert((location.0 + 1, location.1));
            }
            if location.0 > 0 && map[location.0 - 1][location.1] == value + 1 {
                visited.insert((location.0 - 1, location.1));
            }
            if location.1 < map[0].len() - 1 && map[location.0][location.1 + 1] == value + 1 {
                visited.insert((location.0, location.1 + 1));
            }
            if location.1 > 0 && map[location.0][location.1 - 1] == value + 1 {
                visited.insert((location.0, location.1 - 1));
            }
        }
    }
    let mut score = 0;
    for location in visited {
        if map[location.0][location.1] == 9 {
            score += 1;
        }
    }
    score
}

fn rate_trailhead(map: &Vec<Vec<u8>>, trailhead: (usize, usize)) -> u32 {
    let mut possible_paths = HashSet::new();
    let possible_path = vec![trailhead];
    possible_paths.insert(possible_path);
    let mut previous_possible_path_node_count = 0;
    let mut current_possible_path_node_count = node_count(&possible_paths);
    while current_possible_path_node_count > previous_possible_path_node_count {
        previous_possible_path_node_count = current_possible_path_node_count;
        let paths: Vec<_> = possible_paths.iter().cloned().collect();
        for path in paths {
            let last_node = path.last().unwrap();
            let value = map[last_node.0][last_node.1];
            if last_node.0 < map.len() - 1 && map[last_node.0 + 1][last_node.1] == value + 1 {
                let mut new_path = path.clone();
                new_path.push((last_node.0 + 1, last_node.1));
                possible_paths.insert(new_path);
            }
            if last_node.0 > 0 && map[last_node.0 - 1][last_node.1] == value + 1 {
                let mut new_path = path.clone();
                new_path.push((last_node.0 - 1, last_node.1));
                possible_paths.insert(new_path);
            }
            if last_node.1 < map[0].len() - 1 && map[last_node.0][last_node.1 + 1] == value + 1 {
                let mut new_path = path.clone();
                new_path.push((last_node.0, last_node.1 + 1));
                possible_paths.insert(new_path);
            }
            if last_node.1 > 0 && map[last_node.0][last_node.1 - 1] == value + 1 {
                let mut new_path = path.clone();
                new_path.push((last_node.0, last_node.1 - 1));
                possible_paths.insert(new_path);
            }
        }
        current_possible_path_node_count = node_count(&possible_paths);
    }
    let mut rating = 0;
    for path in possible_paths {
        let last_node = path.last().unwrap();
        if map[last_node.0][last_node.1] == 9 {
            rating += 1;
        }
    }

    rating
}

fn node_count(paths: &HashSet<Vec<(usize, usize)>>) -> usize {
    paths.iter().map(|path| path.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
