use std::collections::HashSet;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);

    let mut antinode_locs = HashSet::<(usize, usize)>::new();
    let map_x_bound = map.len() - 1;
    let map_y_bound = map[0].len() - 1;

    for c in ('a'..='z').chain('A'..='Z').chain('0'..='9') {
        let filtered_coords = get_coords_for_char(&map, &c);
        let antinode_coords = locate_antinodes(&filtered_coords, map_x_bound, map_y_bound);
        for node in antinode_coords {
            antinode_locs.insert(node);
        }
    }
    Some(antinode_locs.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);

    let mut antinode_locs = HashSet::<(usize, usize)>::new();
    let map_x_bound = map.len() - 1;
    let map_y_bound = map[0].len() - 1;

    for c in ('a'..='z').chain('A'..='Z').chain('0'..='9') {
        let filtered_coords = get_coords_for_char(&map, &c);
        let antinode_coords = locate_antinodes_part2(&filtered_coords, map_x_bound, map_y_bound);
        for node in antinode_coords {
            antinode_locs.insert(node);
        }
    }
    Some(antinode_locs.len() as u32)
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_coords_for_char(map: &Vec<Vec<char>>, c: &char) -> Vec<(usize, usize)> {
    let mut coords = Vec::new();
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == *c {
                coords.push((row_idx, col_idx));
            }
        }
    }
    coords
}

fn locate_antinodes(coords: &Vec<(usize, usize)>, x_bound: usize, y_bound: usize) -> Vec<(usize, usize)> {
    let mut antinodes = Vec::new();

    // Need at least 2 coordinates to form a line
    if coords.len() < 2 {
        return antinodes;
    }

    // Check each pair of coordinates
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];

            // Calculate the potential antinode coordinates
            // It could be on either side of the line segment

            // Convert to i32 for calculations to handle negative intermediates
            let dx = x2 as i32 - x1 as i32;
            let dy = y2 as i32 - y1 as i32;

            // Check both possible antinode positions
            // One in each direction from the second point
            let potential_antinodes = [
                (
                    (x2 as i32 + dx) as usize,
                    (y2 as i32 + dy) as usize
                ),
                (
                    (x1 as i32 - dx) as usize,
                    (y1 as i32 - dy) as usize
                )
            ];

            for antinode in potential_antinodes {
                // Only include if the coordinates are valid (not negative, which would've wrapped)
                if antinode.0 <= x_bound && antinode.1 <= y_bound {
                    antinodes.push(antinode);
                }
            }
        }
    }

    antinodes
}

fn locate_antinodes_part2(coords: &Vec<(usize, usize)>, x_bound: usize, y_bound: usize) -> Vec<(usize, usize)> {
    let mut antinodes = Vec::new();

    // Need at least 2 coordinates to form a line
    if coords.len() < 2 {
        return antinodes;
    }

    // Check each pair of coordinates
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];

            // Convert to i32 for calculations to handle negative intermediates
            let dx = x2 as i32 - x1 as i32;
            let dy = y2 as i32 - y1 as i32;

            // Check both directions from both points
            // Try multiple steps (multipliers) in each direction
            for multiplier in x_bound as i32*-1..x_bound as i32 {
                let potential_antinode = (
                    (x2 as i32 + dx * multiplier) as usize,
                    (y2 as i32 + dy * multiplier) as usize
                );

                // Only include if the coordinates are valid
                if potential_antinode.0 <= x_bound && potential_antinode.1 <= y_bound {
                    antinodes.push(potential_antinode);
                }

                // Also check from the first point
                let potential_antinode = (
                    (x1 as i32 - dx * multiplier) as usize,
                    (y1 as i32 - dy * multiplier) as usize
                );

                if potential_antinode.0 <= x_bound && potential_antinode.1 <= y_bound {
                    antinodes.push(potential_antinode);
                }
            }
        }
    }

    antinodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
