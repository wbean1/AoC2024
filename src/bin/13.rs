advent_of_code::solution!(13);

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn cost(&self) -> Option<u64> {
        let intersection = get_intersection(
            self.a.1 as f64 / self.a.0 as f64,
            self.b.1 as f64 / self.b.0 as f64,
            self.prize,
        );

        let intersection = (round(intersection.0, 3), round(intersection.1, 3));
        if intersection.0.fract() > 0.001
            || intersection.1.fract() > 0.001
            || intersection.0 < 0.0
            || intersection.1 < 0.0
        {
            return None;
        }

        let a_cost = (intersection.0 as i64 / self.a.0) as u64 * 3;
        let b_cost = ((self.prize.0 - intersection.0 as i64) / self.b.0) as u64;

        Some(a_cost + b_cost)
    }
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}

fn get_intersection(slope_a: f64, slope_b: f64, prize: (i64, i64)) -> (f64, f64) {
    let x = (-slope_b * prize.0 as f64 + prize.1 as f64) / (slope_a - slope_b);
    let y = slope_a * x;

    (x, y)
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
}
