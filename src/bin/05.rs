advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, page_list) = parse_input(input);

    let mut sum = 0;
    for pages in page_list {
        if is_page_order_valid(&pages, &rules) {
            sum += middle_page(&pages);
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, page_list) = parse_input(input);

    let mut sum = 0;
    for pages in page_list {
        if !is_page_order_valid(&pages, &rules) {
            let new_pages = reorder_pages(&pages, &rules);
            sum += middle_page(&new_pages);
        }
    }

    Some(sum)
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let (rules, pages) = input.split_once("\n\n").unwrap();
    let rules: Vec<(u32, u32)> = rules
        .lines()
        .map(|line| {
            let mut parts = line.split('|').map(|s| s.parse().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();
    let page_list: Vec<Vec<u32>> = pages
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();
    (rules, page_list)
}

fn middle_page(pages: &Vec<u32>) -> u32 {
    pages[pages.len() / 2]
}

fn is_page_order_valid(pages: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for rule in rules {
        if pages.contains(&rule.0) && pages.contains(&rule.1) {
            let position_of_first = pages.iter().position(|&p| p == rule.0).unwrap();
            let position_of_second = pages.iter().position(|&p| p == rule.1).unwrap();
            if position_of_first > position_of_second {
                return false;
            }
        }
    }
    true
}

fn reorder_pages(pages: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut new_pages = pages.clone();

    while !is_page_order_valid(&new_pages, &rules) {
        for rule in rules {
            if new_pages.contains(&rule.0) && new_pages.contains(&rule.1) {
                let position_of_first = new_pages.iter().position(|&p| p == rule.0).unwrap();
                let position_of_second = new_pages.iter().position(|&p| p == rule.1).unwrap();
                if position_of_first > position_of_second {
                    new_pages.swap(position_of_first, position_of_second);
                }
            }
        }
    }

    new_pages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_middle_page() {
        assert_eq!(middle_page(&vec![1, 2, 3, 4, 5]), 3);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
