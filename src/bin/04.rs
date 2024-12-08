advent_of_code::solution!(4);

fn is_xmas_going_up(i: usize, j: usize, chars: &Vec<Vec<char>>) -> bool {
    if i < 3 {
        return false;
    }
    chars[i][j] == 'X' && chars[i - 1][j] == 'M' && chars[i - 2][j] == 'A' && chars[i - 3][j] == 'S'
}
fn is_xmas_going_down(i: usize, j: usize, chars: &Vec<Vec<char>>) -> bool {
    if i > chars.len() - 4 {
        return false;
    }
    chars[i][j] == 'X' && chars[i + 1][j] == 'M' && chars[i + 2][j] == 'A' && chars[i + 3][j] == 'S'
}
fn is_xmas_going_right(i: usize, j: usize, chars: &Vec<Vec<char>>) -> bool {
    if j > chars[i].len() - 4 {
        return false;
    }
    chars[i][j] == 'X' && chars[i][j + 1] == 'M' && chars[i][j + 2] == 'A' && chars[i][j + 3] == 'S'
}
fn is_xmas_going_left(i: usize, j: usize, chars: &Vec<Vec<char>>) -> bool {
    if j < 3 {
        return false;
    }
    chars[i][j] == 'X' && chars[i][j - 1] == 'M' && chars[i][j - 2] == 'A' && chars[i][j - 3] == 'S'
}
fn is_xmas_going_diagonal_down_right(i: usize, j: usize, chars: &Vec<Vec<char>>) -> bool {
    if i > chars.len() - 4 || j > chars[i].len() - 4 {
        return false;
    }
    chars[i][j] == 'X'
        && chars[i + 1][j + 1] == 'M'
        && chars[i + 2][j + 2] == 'A'
        && chars[i + 3][j + 3] == 'S'
}
fn is_xmas_going_diagonal_down_left(i: usize, j: usize, chars: &Vec<Vec<char>>) -> bool {
    if i > chars.len() - 4 || j < 3 {
        return false;
    }
    chars[i][j] == 'X'
        && chars[i + 1][j - 1] == 'M'
        && chars[i + 2][j - 2] == 'A'
        && chars[i + 3][j - 3] == 'S'
}
fn is_xmas_going_diagonal_up_right(i: usize, j: usize, chars: &Vec<Vec<char>>) -> bool {
    if i < 3 || j > chars[i].len() - 4 {
        return false;
    }
    chars[i][j] == 'X'
        && chars[i - 1][j + 1] == 'M'
        && chars[i - 2][j + 2] == 'A'
        && chars[i - 3][j + 3] == 'S'
}
fn is_xmas_going_diagonal_up_left(i: usize, j: usize, chars: &Vec<Vec<char>>) -> bool {
    if i < 3 || j < 3 {
        return false;
    }
    chars[i][j] == 'X'
        && chars[i - 1][j - 1] == 'M'
        && chars[i - 2][j - 2] == 'A'
        && chars[i - 3][j - 3] == 'S'
}
fn is_x_mas(i: usize, j: usize, chars: &Vec<Vec<char>>) -> bool {
    if chars[i][j] != 'A' {
        return false;
    }
    if i < 1 || j < 1 || i > chars.len() - 2 || j > chars[i].len() - 2 {
        return false;
    }
    // M.M
    // .A.
    // S.S
    if chars[i - 1][j - 1] == 'M'
        && chars[i + 1][j + 1] == 'S'
        && chars[i - 1][j + 1] == 'M'
        && chars[i + 1][j - 1] == 'S'
    {
        return true;
    }
    // S.S
    // .A.
    // M.M
    if chars[i - 1][j - 1] == 'S'
        && chars[i + 1][j + 1] == 'M'
        && chars[i - 1][j + 1] == 'S'
        && chars[i + 1][j - 1] == 'M'
    {
        return true;
    }
    // S.M
    // .A.
    // S.M
    if chars[i - 1][j - 1] == 'S'
        && chars[i + 1][j + 1] == 'M'
        && chars[i - 1][j + 1] == 'M'
        && chars[i + 1][j - 1] == 'S'
    {
        return true;
    }
    // M.S
    // .A.
    // M.S
    if chars[i - 1][j - 1] == 'M'
        && chars[i + 1][j + 1] == 'S'
        && chars[i - 1][j + 1] == 'S'
        && chars[i + 1][j - 1] == 'M'
    {
        return true;
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count: u32 = 0;

    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j] != 'X' {
                continue;
            }
            if is_xmas_going_up(i, j, &chars) {
                count += 1;
            }
            if is_xmas_going_down(i, j, &chars) {
                count += 1;
            }
            if is_xmas_going_right(i, j, &chars) {
                count += 1;
            }
            if is_xmas_going_left(i, j, &chars) {
                count += 1;
            }
            if is_xmas_going_diagonal_down_right(i, j, &chars) {
                count += 1;
            }
            if is_xmas_going_diagonal_down_left(i, j, &chars) {
                count += 1;
            }
            if is_xmas_going_diagonal_up_right(i, j, &chars) {
                count += 1;
            }
            if is_xmas_going_diagonal_up_left(i, j, &chars) {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count: u32 = 0;

    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            if chars[i][j] != 'A' {
                continue;
            }
            if is_x_mas(i, j, &chars) {
                count += 1;
            }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
