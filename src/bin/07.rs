advent_of_code::solution!(7);

fn has_unsolved_wip(wips: &[Vec<u64>]) -> bool {
    wips.iter().any(|wip| wip.len() > 1)
}

fn get_and_remove_first_unsolved_wip(wips: &mut Vec<Vec<u64>>) -> Vec<u64> {
    let pos = wips.iter().position(|wip| wip.len() > 1).unwrap();
    wips.remove(pos)
}

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
    for equation in equations {
        let mut wips = vec![equation.1];
        while has_unsolved_wip(&wips) {
            let unsolved_wip = get_and_remove_first_unsolved_wip(&mut wips);
            let possible_answer_1 = unsolved_wip[0] + unsolved_wip[1];
            let possible_answer_2 = unsolved_wip[0] * unsolved_wip[1];
            if unsolved_wip.len() == 2 {
                if possible_answer_1 == equation.0 || possible_answer_2 == equation.0 {
                    sum += equation.0;
                    break;
                }
            } else {
                let mut new_wip_1 = vec![possible_answer_1];
                new_wip_1.extend_from_slice(&unsolved_wip[2..]);
                wips.push(new_wip_1);

                let mut new_wip_2 = vec![possible_answer_2];
                new_wip_2.extend_from_slice(&unsolved_wip[2..]);
                wips.push(new_wip_2);
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);

    let mut sum = 0;
    for equation in equations {
        let mut wips = vec![equation.1];
        while has_unsolved_wip(&wips) {
            let unsolved_wip = get_and_remove_first_unsolved_wip(&mut wips);
            let possible_answer_1 = unsolved_wip[0] + unsolved_wip[1];
            let possible_answer_2 = unsolved_wip[0] * unsolved_wip[1];
            let possible_answer_3 = format!("{}{}", unsolved_wip[0], unsolved_wip[1])
                .parse::<u64>()
                .ok();
            if unsolved_wip.len() == 2 {
                if possible_answer_1 == equation.0
                    || possible_answer_2 == equation.0
                    || possible_answer_3.map_or(false, |p3| p3 == equation.0)
                {
                    sum += equation.0;
                    break;
                }
            } else {
                if possible_answer_1 <= equation.0 {
                    let mut new_wip_1 = vec![possible_answer_1];
                    new_wip_1.extend_from_slice(&unsolved_wip[2..]);
                    wips.push(new_wip_1);
                }
                if possible_answer_2 <= equation.0 {
                    let mut new_wip_2 = vec![possible_answer_2];
                    new_wip_2.extend_from_slice(&unsolved_wip[2..]);
                    wips.push(new_wip_2);
                }
                if possible_answer_3.map_or(false, |p3| p3 <= equation.0) {
                    if let Some(p3) = possible_answer_3 {
                        let mut new_wip_3 = vec![p3];
                        new_wip_3.extend_from_slice(&unsolved_wip[2..]);
                        wips.push(new_wip_3);
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
