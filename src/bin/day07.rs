use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let start_pos = lines
        .next()
        .expect("Error")
        .find('S')
        .expect("Error: No start position");

    let (_, _, total_splits) = lines.fold(
        (HashSet::from([start_pos]), HashSet::new(), 0),
        |(mut active, mut buffer, current_splits), line| {
            let line_splits = active
                .drain()
                .fold(0, |acc, pos| match line.as_bytes().get(pos) {
                    Some(b'.') => {
                        buffer.insert(pos);
                        acc
                    }
                    Some(b'^') => {
                        buffer.insert(pos - 1);
                        buffer.insert(pos + 1);
                        acc + 1
                    }
                    _ => todo!("What!"),
                });

            (buffer, active, current_splits + line_splits)
        },
    );

    total_splits
}

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let start_pos = lines
        .next()
        .expect("Error")
        .find('S')
        .expect("Error: No start position");

    let (final_map, _) = lines.fold(
        (HashMap::from([(start_pos, 1)]), HashMap::new()),
        |(mut active, mut buffer), line| {
            active
                .drain()
                .for_each(|(pos, count)| match line.as_bytes().get(pos) {
                    Some(b'.') => {
                        *buffer.entry(pos).or_insert(0) += count;
                    }
                    Some(b'^') => {
                        *buffer.entry(pos - 1).or_insert(0) += count;
                        *buffer.entry(pos + 1).or_insert(0) += count;
                    }
                    _ => todo!("What!"),
                });

            (buffer, active)
        },
    );

    final_map.values().sum()
}

fn main() {
    let input = include_str!("../../inputs/day07.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../inputs/day07_test.txt");
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day07_test.txt");
        assert_eq!(part2(input), 40);
    }
}
