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

    let mut sum = 0;
    for (answer, equation) in equations {
        let mut eqs = vec![equation];
        while let Some(unsolved_eq) = eqs.pop() {
            let possible_answer_1 = unsolved_eq[0] + unsolved_eq[1];
            let possible_answer_2 = unsolved_eq[0] * unsolved_eq[1];
            if unsolved_eq.len() == 2 { // len 2 means these are our only possible answers for this eq
                if possible_answer_1 == answer || possible_answer_2 == answer {
                    sum += answer;
                    break;
                } else {
                    continue;
                }
            } else {
                let mut new_eq_1 = vec![possible_answer_1];
                new_eq_1.extend_from_slice(&unsolved_eq[2..]);
                eqs.push(new_eq_1);

                let mut new_eq_2 = vec![possible_answer_2];
                new_eq_2.extend_from_slice(&unsolved_eq[2..]);
                eqs.push(new_eq_2);
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    let mut sum = 0;
    for (answer, equation) in equations {
        let mut eqs = vec![equation];
        while let Some(unsolved_eq) = eqs.pop() {
            let possible_answer_1 = unsolved_eq[0] + unsolved_eq[1];
            let possible_answer_2 = unsolved_eq[0] * unsolved_eq[1];
            let possible_answer_3 = format!("{}{}", unsolved_eq[0], unsolved_eq[1])
                .parse::<u64>()
                .ok();
            if unsolved_eq.len() == 2 {
                if possible_answer_1 == answer
                    || possible_answer_2 == answer
                    || possible_answer_3.map_or(false, |p3| p3 == answer)
                {
                    sum += answer;
                    break;
                } else {
                    continue;
                }
            } else {
                if possible_answer_1 <= answer {
                    let mut new_eq_1 = vec![possible_answer_1];
                    new_eq_1.extend_from_slice(&unsolved_eq[2..]);
                    eqs.push(new_eq_1);
                }
                if possible_answer_2 <= answer {
                    let mut new_eq_2 = vec![possible_answer_2];
                    new_eq_2.extend_from_slice(&unsolved_eq[2..]);
                    eqs.push(new_eq_2);
                }
                if possible_answer_3.map_or(false, |p3| p3 <= answer) {
                    if let Some(p3) = possible_answer_3 {
                        let mut new_eq_3 = vec![p3];
                        new_eq_3.extend_from_slice(&unsolved_eq[2..]);
                        eqs.push(new_eq_3);
                    }
                }
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
