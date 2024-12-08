advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let (id_part, numbers_part) = line.split_once(':').unwrap();
            let id = id_part.trim().parse().unwrap();
            let numbers: Vec<u64> = numbers_part
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            (id, numbers)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    Some(
        equations
            .into_iter()
            .filter(|(answer, equation)| solve_equation_part1(equation, *answer))
            .map(|(answer, _)| answer)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    Some(
        equations
            .into_iter()
            .filter(|(answer, equation)| solve_equation_part2(equation, *answer))
            .map(|(answer, _)| answer)
            .sum(),
    )
}

fn solve_equation_part1(equation: &[u64], target: u64) -> bool {
    if equation.len() == 2 {
        let sum = equation[0] + equation[1];
        let product = equation[0] * equation[1];
        return sum == target || product == target;
    }

    let sum = equation[0] + equation[1];
    let product = equation[0] * equation[1];

    let mut new_eq_1 = vec![sum];
    new_eq_1.extend_from_slice(&equation[2..]);

    let mut new_eq_2 = vec![product];
    new_eq_2.extend_from_slice(&equation[2..]);

    solve_equation_part1(&new_eq_1, target) || solve_equation_part1(&new_eq_2, target)
}

fn solve_equation_part2(equation: &[u64], target: u64) -> bool {
    if equation.len() == 2 {
        let sum = equation[0] + equation[1];
        let product = equation[0] * equation[1];
        let concat = format!("{}{}", equation[0], equation[1])
            .parse::<u64>()
            .ok();
        return sum == target || product == target || concat.map_or(false, |c| c == target);
    }

    let sum = equation[0] + equation[1];
    let product = equation[0] * equation[1];
    let concat = format!("{}{}", equation[0], equation[1])
        .parse::<u64>()
        .ok();

    let mut results = false;

    if sum <= target {
        let mut new_eq = vec![sum];
        new_eq.extend_from_slice(&equation[2..]);
        results |= solve_equation_part2(&new_eq, target);
    }

    if product <= target && !results {
        let mut new_eq = vec![product];
        new_eq.extend_from_slice(&equation[2..]);
        results |= solve_equation_part2(&new_eq, target);
    }

    if let Some(c) = concat {
        if c <= target && !results {
            let mut new_eq = vec![c];
            new_eq.extend_from_slice(&equation[2..]);
            results |= solve_equation_part2(&new_eq, target);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
