fn parse_bank_n(input: &str, n: usize) -> i64 {
    let digits: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();

    let mut result: i64 = 0;
    let mut current_pos = 0;

    for i in 0..n {
        let remaining_needed = n - 1 - i;
        let limit = digits.len() - remaining_needed;

        let search_slice = &digits[current_pos..limit];

        if let Some((offset_idx, &val)) = search_slice
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|&(_, v)| v)
        {
            result = result * 10 + (val as i64);
            current_pos += offset_idx + 1;
        } else {
            return 0;
        }
    }

    result
}

fn part1(input: &str) -> i64 {
    input.lines().map(|line| parse_bank_n(line, 2)).sum()
}

fn part2(input: &str) -> i64 {
    input.lines().map(|line| parse_bank_n(line, 12)).sum()
}

fn main() {
    let input = include_str!("../../inputs/day03.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../inputs/day03_test.txt");
        assert_eq!(part1(input), 357);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day03_test.txt");
        assert_eq!(part2(input), 3121910778619);
    }
}
