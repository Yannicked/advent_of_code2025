use std::ops::RangeInclusive;
use std::cmp;

fn merge_and_sum_ranges(ranges: &mut Vec<RangeInclusive<i64>>) -> i64 {
    ranges.sort_unstable_by_key(|r| *r.start());

    let mut total_length = 0;
    let mut current_start = ranges[0].start();
    let mut current_end = ranges[0].end();

    for range in ranges.iter().skip(1) {
        if range.start() <= current_end {
            current_end = cmp::max(current_end, range.end());
        } else {
            total_length += current_end - current_start + 1;
            current_start = range.start();
            current_end = range.end();
        }
    }

    total_length += current_end - current_start + 1;

    total_length
}

fn parse_ranges(input: &str) -> Vec<RangeInclusive<i64>> {
    input
        .lines()
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            Some(start.parse::<i64>().ok()?..=end.parse::<i64>().ok()?)
        })
        .collect()
}

fn part1(input: &str) -> i64 {
    let (ranges_block, ids_block) = input.split_once("\n\n").unwrap_or((input, ""));

    let ranges = parse_ranges(ranges_block);

    ids_block
        .lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count() as i64
}

fn part2(input: &str) -> i64 {
    let (ranges_block, _) = input.split_once("\n\n").unwrap_or((input, ""));
    let mut ranges = parse_ranges(ranges_block);
    merge_and_sum_ranges(&mut ranges)
}

fn main() {
    let input = include_str!("../../inputs/day05.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../inputs/day05_test.txt");
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day05_test.txt");
        assert_eq!(part2(input), 14);
    }
}
