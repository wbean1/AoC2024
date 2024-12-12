advent_of_code::solution!(12);
use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_regions(map: &Vec<Vec<char>>) -> HashMap<char, HashSet<(usize, usize)>> {
    let mut regions = HashMap::new();
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            regions
                .entry(ch)
                .or_insert(HashSet::new())
                .insert((row_idx, col_idx));
        }
    }
    regions
}

fn contiguous_regions(region: &HashSet<(usize, usize)>) -> Vec<HashSet<(usize, usize)>> {
    let mut sub_regions = Vec::<HashSet<(usize, usize)>>::new();
    let mut visited = HashSet::new();
    for &(row_idx, col_idx) in region {
        if visited.contains(&(row_idx, col_idx)) {
            continue;
        }
        let sub_region = bfs(region, (row_idx, col_idx));
        sub_regions.push(sub_region.clone());
        visited.extend(sub_region);
    }
    sub_regions
}

fn calculate_price(region: &HashSet<(usize, usize)>) -> u32 {
    let sub_regions = contiguous_regions(region);

    let mut total_price = 0;
    // now we can calculate the price for each sub-region
    for sub_region in sub_regions {
        let area = sub_region.len() as u32;
        let mut perimeter = 4 * area;
        for &(row_idx, col_idx) in &sub_region {
            if row_idx > 0 && sub_region.contains(&(row_idx - 1, col_idx)) {
                perimeter -= 1;
            }
            if sub_region.contains(&(row_idx + 1, col_idx)) {
                perimeter -= 1;
            }
            if col_idx > 0 && sub_region.contains(&(row_idx, col_idx - 1)) {
                perimeter -= 1;
            }
            if sub_region.contains(&(row_idx, col_idx + 1)) {
                perimeter -= 1;
            }
        }
        let price = area * perimeter;
        total_price += price;
    }

    total_price
}

fn calculate_price_part2(region: &HashSet<(usize, usize)>) -> u32 {
    // first, we need to split the region into its connected components
    let sub_regions = contiguous_regions(region);

    let mut total_price = 0;
    for sub_region in sub_regions {
        let area = sub_region.len() as u32;
        // count the corners of the sub-region to get number of edges
        let corners_count = count_corners(&sub_region);

        let price = area * corners_count;
        total_price += price;
    }

    total_price
}

fn count_corners(region: &HashSet<(usize, usize)>) -> u32 {
    let mut corners_count = 0;
    for &(row_idx, col_idx) in region {
        let n = row_idx > 0 && region.contains(&(row_idx - 1, col_idx));
        let e = region.contains(&(row_idx, col_idx + 1));
        let s = region.contains(&(row_idx + 1, col_idx));
        let w = col_idx > 0 && region.contains(&(row_idx, col_idx - 1));
        let ne = row_idx > 0 && region.contains(&(row_idx - 1, col_idx + 1));
        let se = region.contains(&(row_idx + 1, col_idx + 1));
        let sw = col_idx > 0 && region.contains(&(row_idx + 1, col_idx - 1));
        let nw = row_idx > 0 && col_idx > 0 && region.contains(&(row_idx - 1, col_idx - 1));
        // ways it can be 4 convex corners
        if !n && !e && !s && !w {
            corners_count += 4;

            continue;
        }
        // ways it can be 4,3,2,1 inverse corners
        if n && e && s && w {
            let mut inner_count = 0;
            if !ne {
                inner_count += 1;
            }
            if !se {
                inner_count += 1;
            }
            if !sw {
                inner_count += 1;
            }
            if !nw {
                inner_count += 1;
            }
            corners_count += inner_count;

            continue;
        }
        // additional ways it can be 2 inverse corners
        if !n && e && s && w {
            let mut inner_count = 0;
            if !se {
                inner_count += 1;
            }
            if !sw {
                inner_count += 1;
            }
            corners_count += inner_count;

            continue;
        } else if n && !e && s && w {
            let mut inner_count = 0;
            if !nw {
                inner_count += 1;
            }
            if !sw {
                inner_count += 1;
            }
            corners_count += inner_count;

            continue;
        } else if n && e && !s && w {
            let mut inner_count = 0;
            if !ne {
                inner_count += 1;
            }
            if !nw {
                inner_count += 1;
            }
            corners_count += inner_count;

            continue;
        } else if n && e && s && !w {
            let mut inner_count = 0;
            if !ne {
                inner_count += 1;
            }
            if !se {
                inner_count += 1;
            }
            corners_count += inner_count;

            continue;
        }
        // ways it can be 2 convex corners
        if !n && !e && !s && w {
            corners_count += 2;

            continue;
        } else if n && !e && !s && !w {
            corners_count += 2;

            continue;
        } else if !n && !e && s && !w {
            corners_count += 2;

            continue;
        } else if !n && e && !s && !w {
            corners_count += 2;

            continue;
        } else if !n && !e && s && w && !sw {
            corners_count += 2;

            continue;
        } else if n && !e && !s && w && !nw {
            corners_count += 2;

            continue;
        } else if n && e && !s && !w && !ne {
            corners_count += 2;

            continue;
        } else if !n && e && s && !w && !se {
            corners_count += 2;

            continue;
        }
        // ways it can be 1 convex corner
        if !n && !e && s && w {
            corners_count += 1;

            continue;
        } else if n && !e && !s && w {
            corners_count += 1;

            continue;
        } else if n && e && !s && !w {
            corners_count += 1;

            continue;
        } else if !n && e && s && !w {
            corners_count += 1;

            continue;
        }

        // everything else should be 0 corners...
    }
    corners_count
}

fn bfs(region: &HashSet<(usize, usize)>, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut queue = vec![start];

    while let Some((row, col)) = queue.pop() {
        if !visited.insert((row, col)) {
            continue;
        }

        for (nr, nc) in [
            (row.wrapping_sub(1), col),
            (row + 1, col),
            (row, col.wrapping_sub(1)),
            (row, col + 1),
        ] {
            if region.contains(&(nr, nc)) {
                queue.push((nr, nc));
            }
        }
    }
    visited
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let regions = find_regions(&map);
    let mut total_price = 0;
    for (_, value) in regions {
        let price = calculate_price(&value);
        total_price += price;
    }
    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let regions = find_regions(&map);
    let mut total_price = 0;
    for (_, value) in regions {
        let price = calculate_price_part2(&value);
        total_price += price;
    }
    Some(total_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
