use std::iter::zip;

struct Columns<I> {
    iters: Vec<I>,
}

impl<I> Columns<I> {
    fn new(iters: Vec<I>) -> Self {
        Self { iters }
    }
}

impl<I: Iterator> Iterator for Columns<I> {
    type Item = Vec<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iters.iter_mut().map(|iter| iter.next()).collect()
    }
}

fn part1(input: &str) -> i64 {
    let mut lines = input.lines();

    let operators_line = lines.next_back().expect("Input empty");
    let operators = operators_line.split_whitespace();

    let rows: Vec<Vec<i64>> = lines
        .map(|line| {
            line.split_whitespace()
                .filter_map(|n| n.parse::<i64>().ok())
                .collect()
        })
        .collect();

    let col_iters: Vec<_> = rows.into_iter().map(|row| row.into_iter()).collect();

    let transposed = Columns::new(col_iters);

    zip(operators, transposed)
        .map(|(op, nums)| match op {
            "+" => nums.iter().sum::<i64>(),
            _ => nums.iter().product::<i64>(),
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let mut lines = input.lines();

    let operators_line = lines.next_back().expect("Input empty");
    let operators = operators_line.split_whitespace();

    let rows: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();

    let col_iters: Vec<_> = rows.into_iter().map(|row| row.into_iter()).collect();

    let transposed_results: Vec<Result<i64, _>> = Columns::new(col_iters)
        .map(|chars| {
            chars
                .into_iter()
                .collect::<String>()
                .trim()
                .parse::<i64>()
        })
        .collect();

    let operand_groups = transposed_results
        .split(|res| res.is_err())
        .map(|group| group.iter().filter_map(|res| res.as_ref().ok()));

    zip(operators, operand_groups)
        .map(|(op, nums)| match op {
            "+" => nums.sum::<i64>(),
            _ => nums.product::<i64>(),
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/day06.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../inputs/day06_test.txt");
        assert_eq!(part1(input), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day06_test.txt");
        assert_eq!(part2(input), 3263827);
    }
}
