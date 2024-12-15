use itertools::Itertools;

advent_of_code::solution!(15);

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let (map_str, move_str) = input.split("\n\n").collect_tuple().unwrap();
    let map: Vec<Vec<char>> = map_str.lines().map(|line| line.chars().collect()).collect();
    let moves: Vec<char> = move_str
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect();
    (map, moves)
}

fn tick(map: &Vec<Vec<char>>, moove: char) -> Vec<Vec<char>> {
    let robot = find_robot(map);
    if moove == '>' {
        // move right
        if can_move_right(map, robot) {
            return move_right(map, robot);
        }
    } else if moove == 'v' {
        // move down
        if can_move_down(map, robot) {
            return move_down(map, robot);
        }
    } else if moove == '<' {
        // move left
        if can_move_left(map, robot) {
            return move_left(map, robot);
        }
    } else if moove == '^' {
        // move up
        if can_move_up(map, robot) {
            return move_up(map, robot);
        }
    }
    map.clone()
}

fn can_move_right(map: &Vec<Vec<char>>, robot: (usize, usize)) -> bool {
    if robot.1 == map[0].len() - 1 {
        return false;
    }
    if map[robot.0][robot.1 + 1] == '#' {
        return false;
    }
    if map[robot.0][robot.1 + 1] == '.' {
        return true;
    }
    can_move_right(map, (robot.0, robot.1 + 1))
}

fn can_move_down(map: &Vec<Vec<char>>, robot: (usize, usize)) -> bool {
    if robot.0 == map.len() - 1 {
        return false;
    }
    if map[robot.0 + 1][robot.1] == '#' {
        return false;
    }
    if map[robot.0 + 1][robot.1] == '.' {
        return true;
    }
    can_move_down(map, (robot.0 + 1, robot.1))
}

fn can_move_left(map: &Vec<Vec<char>>, robot: (usize, usize)) -> bool {
    if robot.1 == 0 {
        return false;
    }
    if map[robot.0][robot.1 - 1] == '#' {
        return false;
    }
    if map[robot.0][robot.1 - 1] == '.' {
        return true;
    }
    can_move_left(map, (robot.0, robot.1 - 1))
}

fn can_move_up(map: &Vec<Vec<char>>, robot: (usize, usize)) -> bool {
    if robot.0 == 0 {
        return false;
    }
    if map[robot.0 - 1][robot.1] == '#' {
        return false;
    }
    if map[robot.0 - 1][robot.1] == '.' {
        return true;
    }
    can_move_up(map, (robot.0 - 1, robot.1))
}

fn find_robot(map: &[Vec<char>]) -> (usize, usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '@' {
                return (y, x);
            }
        }
    }
    panic!("No robot found");
}

fn score(map: &[Vec<char>]) -> u32 {
    let boxes = find_boxes(map);
    let mut sum = 0;
    for i in boxes {
        sum += i.1 as u32 * 100 + i.0 as u32;
    }

    sum
}

fn find_boxes(map: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut boxes = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 'O' {
                boxes.push((x, y));
            }
        }
    }
    boxes
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, moves) = parse_input(input);
    for moove in moves {
        map = tick(&map, moove);
    }

    Some(score(&map))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn move_right(map: &[Vec<char>], robot: (usize, usize)) -> Vec<Vec<char>> {
    let mut new_map = map.to_owned();
    new_map[robot.0][robot.1 + 1] = '@';
    new_map[robot.0][robot.1] = '.';
    let mut have_swapped = map[robot.0][robot.1 + 1] == '.';
    let mut current_pos = (robot.0, robot.1 + 1);
    while !have_swapped {
        if map[current_pos.0][current_pos.1] == '.' {
            new_map[current_pos.0][current_pos.1] = map[current_pos.0][current_pos.1 - 1];
            have_swapped = true;
        } else {
            new_map[current_pos.0][current_pos.1] = map[current_pos.0][current_pos.1 - 1];
            current_pos = (current_pos.0, current_pos.1 + 1);
        }
    }
    new_map
}

fn move_down(map: &[Vec<char>], robot: (usize, usize)) -> Vec<Vec<char>> {
    let mut new_map = map.to_owned();
    new_map[robot.0 + 1][robot.1] = '@';
    new_map[robot.0][robot.1] = '.';
    let mut have_swapped = map[robot.0 + 1][robot.1] == '.';
    let mut current_pos = (robot.0 + 1, robot.1);
    while !have_swapped {
        if map[current_pos.0][current_pos.1] == '.' {
            new_map[current_pos.0][current_pos.1] = map[current_pos.0 - 1][current_pos.1];
            have_swapped = true;
        } else {
            new_map[current_pos.0][current_pos.1] = map[current_pos.0 - 1][current_pos.1];
            current_pos = (current_pos.0 + 1, current_pos.1);
        }
    }
    new_map
}

fn move_left(map: &[Vec<char>], robot: (usize, usize)) -> Vec<Vec<char>> {
    let mut new_map = map.to_owned();
    new_map[robot.0][robot.1 - 1] = '@';
    new_map[robot.0][robot.1] = '.';
    let mut have_swapped = map[robot.0][robot.1 - 1] == '.';
    let mut current_pos = (robot.0, robot.1 - 1);
    while !have_swapped {
        if map[current_pos.0][current_pos.1] == '.' {
            new_map[current_pos.0][current_pos.1] = map[current_pos.0][current_pos.1 + 1];
            have_swapped = true;
        } else {
            new_map[current_pos.0][current_pos.1] = map[current_pos.0][current_pos.1 + 1];
            current_pos = (current_pos.0, current_pos.1 - 1);
        }
    }
    new_map
}

fn move_up(map: &[Vec<char>], robot: (usize, usize)) -> Vec<Vec<char>> {
    let mut new_map = map.to_owned();
    new_map[robot.0 - 1][robot.1] = '@';
    new_map[robot.0][robot.1] = '.';
    let mut have_swapped = map[robot.0 - 1][robot.1] == '.';
    let mut current_pos = (robot.0 - 1, robot.1);
    while !have_swapped {
        if map[current_pos.0][current_pos.1] == '.' {
            new_map[current_pos.0][current_pos.1] = map[current_pos.0 + 1][current_pos.1];
            have_swapped = true;
        } else {
            new_map[current_pos.0][current_pos.1] = map[current_pos.0 + 1][current_pos.1];
            current_pos = (current_pos.0 - 1, current_pos.1);
        }
    }
    new_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_move_right() {
        let map = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '@', 'O', '.', '#'],
        ];
        let robot = (1, 6);
        let new_map = move_right(&map, robot);
        assert_eq!(
            new_map,
            vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                vec!['.', '.', '.', '.', '.', '.', '.', '@', 'O', '#'],
            ]
        );
    }

    #[test]
    fn test_move_down() {
        let map = vec![
            vec!['.', '.', '.', '.', '@', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', 'O', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', 'O', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
        ];
        let robot = (0, 4);
        let new_map = move_down(&map, robot);
        assert_eq!(
            new_map,
            vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                vec!['.', '.', '.', '.', '@', '.', '.', '.', '.', '#'],
                vec!['.', '.', '.', '.', 'O', '.', '.', '.', '.', '#'],
                vec!['.', '.', '.', '.', 'O', '.', '.', '.', '.', '#'],
            ]
        );
    }

    #[test]
    fn test_move_left() {
        let map = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', 'O', 'O', '@', '.', '.', '.', '.', '#'],
        ];
        let robot = (1, 4);
        let new_map = move_left(&map, robot);
        assert_eq!(
            new_map,
            vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
                vec!['.', 'O', 'O', '@', '.', '.', '.', '.', '.', '#'],
            ]
        );
    }

    #[test]
    fn test_move_up() {
        let map = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', 'O', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', 'O', '.', '.', '.', '.', '#'],
            vec!['.', '.', '.', '.', '@', '.', '.', '.', '.', '#'],
        ];
        let robot = (3, 4);
        let new_map = move_up(&map, robot);
        assert_eq!(
            new_map,
            vec![
                vec!['.', '.', '.', '.', 'O', '.', '.', '.', '.', '#'],
                vec!['.', '.', '.', '.', 'O', '.', '.', '.', '.', '#'],
                vec!['.', '.', '.', '.', '@', '.', '.', '.', '.', '#'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#'],
            ]
        );
    }
}
