fn part1(input: &str) -> String {
    "todo".to_string()
}

fn part2(input: &str) -> String {
    "todo".to_string()
}

fn main() {
    let input = include_str!("../../inputs/day01.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(part1(input), "todo");
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), "todo");
    }
}
