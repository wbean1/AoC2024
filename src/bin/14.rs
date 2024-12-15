advent_of_code::solution!(14);
use lazy_static::lazy_static;

// TODO: This is a hack to make the code work for the example.
// lazy_static! {
//     static ref MAX_X: i64 = 10;
//     static ref MAX_Y: i64 = 6;
// }

lazy_static! {
    static ref MAX_X: i64 = 100;
    static ref MAX_Y: i64 = 102;
}

#[derive(Debug)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Split the line into position and velocity parts
            let parts: Vec<&str> = line.split(' ').collect();

            // Parse position
            let pos_str = parts[0].trim_start_matches("p=");
            let pos_nums: Vec<i64> = pos_str.split(',').map(|n| n.parse().unwrap()).collect();

            // Parse velocity
            let vel_str = parts[1].trim_start_matches("v=");
            let vel_nums: Vec<i64> = vel_str.split(',').map(|n| n.parse().unwrap()).collect();

            Robot {
                position: (pos_nums[0], pos_nums[1]),
                velocity: (vel_nums[0], vel_nums[1]),
            }
        })
        .collect()
}

fn tick(robots: &[Robot]) -> Vec<Robot> {
    robots
        .iter()
        .map(|robot| {
            let mut new_position = (
                robot.position.0 + robot.velocity.0,
                robot.position.1 + robot.velocity.1,
            );
            if new_position.0 < 0 {
                new_position.0 = *MAX_X + new_position.0 + 1;
            }
            if new_position.1 < 0 {
                new_position.1 = *MAX_Y + new_position.1 + 1;
            }
            if new_position.0 > *MAX_X {
                new_position.0 = new_position.0 - *MAX_X - 1;
            }
            if new_position.1 > *MAX_Y {
                new_position.1 = new_position.1 - *MAX_Y - 1;
            }
            Robot {
                position: new_position,
                velocity: robot.velocity,
            }
        })
        .collect()
}

fn score(robots: &Vec<Robot>) -> u64 {
    let left_right_boundary = *MAX_X / 2;
    let top_bottom_boundary = *MAX_Y / 2;
    let mut upper_left = 0;
    let mut upper_right = 0;
    let mut lower_left = 0;
    let mut lower_right = 0;

    for robot in robots {
        if robot.position.0 < left_right_boundary {
            if robot.position.1 < top_bottom_boundary {
                upper_left += 1;
            } else if robot.position.1 > top_bottom_boundary {
                lower_left += 1;
            }
        }
        if robot.position.0 > left_right_boundary {
            if robot.position.1 < top_bottom_boundary {
                upper_right += 1;
            } else if robot.position.1 > top_bottom_boundary {
                lower_right += 1;
            }
        }
    }

    upper_left * upper_right * lower_left * lower_right
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots = parse_input(input);
    for _i in 0..100 {
        robots = tick(&robots);
    }

    Some(score(&robots) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = parse_input(input);
    let mut seconds = 0;
    for _ in 1..101 {
        robots = tick(&robots);
        seconds += 1;
        if easter_egg_print(seconds, &robots) {
            break;
        };
    }

    Some(seconds)
}

fn easter_egg_print(seconds: u32, robots: &Vec<Robot>) -> bool {
    println!("second: {}", seconds);
    let mut grid = vec![vec![0u32; (*MAX_X + 1) as usize]; (*MAX_Y + 1) as usize];

    for robot in robots {
        grid[robot.position.1 as usize][robot.position.0 as usize] += 1;
    }

    for row in grid {
        for cell in row {
            if cell == 0 {
                print!(".");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
    println!();

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
