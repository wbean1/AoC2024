advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

impl Machine {
    fn cost(&self) -> Option<u64> {
        let intersection = get_intersection(
            self.a.1 as f64 / self.a.0 as f64,
            self.b.1 as f64 / self.b.0 as f64,
            self.prize,
        );

        if intersection.is_none() {
            None
        } else {
            let (x, _y) = intersection.unwrap();
            let a_cost = (x.round() as u64 / self.a.0) * 3;
            let b_cost = (self.prize.0 - x.round() as u64) / self.b.0;
            Some(a_cost + b_cost)
        }
    }
}

fn get_intersection(slope_a: f64, slope_b: f64, prize: (u64, u64)) -> Option<(f64, f64)> {
    let x = (-slope_b * prize.0 as f64 + prize.1 as f64) / (slope_a - slope_b);
    let y = slope_a * x;

    // check if the intersection is valid for the machine, that is:
    // 1. the intersection is not negative
    if x < -0.01 || y < -0.01 {
        return None;
    }

    // 2. the intersection is not greater than the prize
    if x.round() as u64 > prize.0 {
        return None;
    }

    // 3. the intersection occurs on (or within floating point precision) of whole numbers
    if (x.fract() > 0.001 && x.fract() < 0.999) || (y.fract() > 0.001 && y.fract() < 0.999) {
        return None;
    }

    Some((x, y))
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|machine_str| {
            let lines: Vec<&str> = machine_str.lines().collect();
            let parse_coords = |line: &str| {
                let parts: Vec<&str> = line.split(": ").nth(1).unwrap().split(", ").collect();
                let x = parts[0]
                    .trim_start_matches('X')
                    .trim_start_matches('=')
                    .trim_start_matches('+')
                    .parse()
                    .unwrap();
                let y = parts[1]
                    .trim_start_matches('Y')
                    .trim_start_matches('=')
                    .trim_start_matches('+')
                    .parse()
                    .unwrap();
                (x, y)
            };

            Machine {
                a: parse_coords(lines[0]),
                b: parse_coords(lines[1]),
                prize: parse_coords(lines[2]),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_machines(input);
    Some(
        machines
            .into_iter()
            .map(|machine| {
                let cost = machine.cost();
                println!("machine: {:?}, cost: {:?}", machine, cost);
                cost.unwrap_or(0)
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse_machines(input);
    Some(
        machines
            .into_iter()
            .map(|mut machine| {
                machine.prize.0 += 10000000000000;
                machine.prize.1 += 10000000000000;
                let cost = machine.cost();
                println!("machine: {:?}, cost: {:?}", machine, cost);
                cost.unwrap_or(0)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
