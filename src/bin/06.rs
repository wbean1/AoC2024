advent_of_code::solution!(6);
use std::collections::HashSet;

fn get_start_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == '^' {
                return (x, y);
            }
        }
    }
    panic!("No starting position (^) found in map");
}

fn turn_right(direction: char) -> char {
    match direction {
        'N' => 'E',
        'E' => 'S',
        'S' => 'W',
        'W' => 'N',
        _ => panic!("unsafe direction"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.trim().lines().map(|line| line.chars().collect()).collect();
    let mut visited = HashSet::new();
    let mut is_on_map = true;
    let mut current_position = get_start_position(&map);
    let mut direction = 'N';

    while is_on_map {
        visited.insert(current_position);

        // see if can move in the current direction
        let mut new_position = (current_position.0 as i32, current_position.1 as i32);
        match direction {
            'N' => new_position.1 -= 1,
            'E' => new_position.0 += 1,
            'S' => new_position.1 += 1,
            'W' => new_position.0 -= 1,
            _ => panic!("unsafe direction"),
        }

        if new_position.1 < 0 || new_position.1 >= map.len() as i32 || new_position.0 < 0 || new_position.0 >= map[0].len() as i32 {
            is_on_map = false;
        } else {
            if map[new_position.1 as usize][new_position.0 as usize] == '#' {
                direction = turn_right(direction);
            } else {
                current_position = (new_position.0 as usize, new_position.1 as usize);
            }
        }
    }

    Some(visited.len() as u32)

}

pub fn part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<char>> = input.trim().lines().map(|line| line.chars().collect()).collect();
    let mut visited = HashSet::new();
    let mut is_on_map = true;
    let mut current_position = get_start_position(&map);
    let mut direction = 'N';

    // run through part 1 to get list of visited positions
    //  these are the positions we'll iterate through
    //  changing to see if loops form
    while is_on_map {
        visited.insert(current_position);

        // see if can move in the current direction
        let mut new_position = (current_position.0 as i32, current_position.1 as i32);
        match direction {
            'N' => new_position.1 -= 1,
            'E' => new_position.0 += 1,
            'S' => new_position.1 += 1,
            'W' => new_position.0 -= 1,
            _ => panic!("unsafe direction"),
        }

        if new_position.1 < 0 || new_position.1 >= map.len() as i32 || new_position.0 < 0 || new_position.0 >= map[0].len() as i32 {
            is_on_map = false;
        } else {
            if map[new_position.1 as usize][new_position.0 as usize] == '#' {
                direction = turn_right(direction);
            } else {
                current_position = (new_position.0 as usize, new_position.1 as usize);
            }
        }
    }

    let mut loops_formed = 0;
    for position in visited {
        if forms_loop_when_blocked(&map, position) {
            loops_formed += 1;
        }
    }

    Some(loops_formed)
}

fn forms_loop_when_blocked(map: &Vec<Vec<char>>, blocked_pos: (usize, usize)) -> bool {
    let mut visited = HashSet::new();
    let mut current_position = get_start_position(&map);
    let mut direction = 'N';

    loop {
        // Store both position and direction in visited set
        if !visited.insert((current_position, direction)) {
            // If we couldn't insert because this exact state was seen before, it's a loop
            return true;
        }

        // Calculate next position based on current direction
        let mut new_position = (current_position.0 as i32, current_position.1 as i32);
        match direction {
            'N' => new_position.1 -= 1,
            'E' => new_position.0 += 1,
            'S' => new_position.1 += 1,
            'W' => new_position.0 -= 1,
            _ => panic!("unsafe direction"),
        }

        // Check if we're going off the map
        if new_position.1 < 0 || new_position.1 >= map.len() as i32 ||
           new_position.0 < 0 || new_position.0 >= map[0].len() as i32 {
            return false; // Path goes off map, no loop formed
        }

        let new_pos_usize = (new_position.0 as usize, new_position.1 as usize);

        // Check if we hit our blocked position or an existing wall
        if new_pos_usize == blocked_pos || map[new_pos_usize.1][new_pos_usize.0] == '#' {
            direction = turn_right(direction);
        } else {
            current_position = new_pos_usize;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
