use advent_of_code_2025::utils::read_lines;

fn parse_line(input: &str) -> Option<i64>{
    let mut chars = input.chars();

    if let Some(first) = chars.next() {
        let rest = chars.as_str();
        let number: i64 = rest.parse().ok()?;
        
        if first == 'R' {
            return Some(number);
        } else {
            return Some(-number);
        }
    }
    return None;
}

fn turn_dial(dial: i64, change: i64) -> i64 {
    let dial_pos = dial + change;
    return dial_pos.rem_euclid(100);
}

fn turn_dial_counter(dial: i64, change: i64) -> (i64, i64) {
    let mut counter = change.abs() / 100;

    let remainder = change % 100;
    
    let new_pos = (dial + remainder).rem_euclid(100);

    if remainder > 0 {
        if new_pos == 0 || new_pos < dial {
            counter += 1;
        }
    } else if remainder < 0 {
        if new_pos == 0 {
            counter += 1;
        } else if new_pos > dial && dial != 0 {
            counter += 1;
        }
    }

    return (new_pos, counter)
}

fn part1(input: &str) -> i64 {
    let mut dial: i64 = 50;
    let mut password: i64 = 0;
    for line in read_lines(input).iter() {
        if let Some(change) = parse_line(line) {
            dial = turn_dial(dial, change);
            if dial == 0 {
                password += 1;
            }
        } else {
            panic!("ERROR {}", line);
        }
    }
    return password;
}

fn part2(input: &str) -> i64 {
    let mut dial: i64 = 50;
    let mut counter: i64;
    let mut password: i64 = 0;
    for line in read_lines(input).iter() {
        if let Some(change) = parse_line(line) {
            (dial, counter) = turn_dial_counter(dial, change);
            password += counter
        } else {
            panic!("ERROR {}", line);
        }
    }
    return password;
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
        let input = include_str!("../../inputs/day01_test.txt");
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day01_test.txt");
        assert_eq!(part2(input), 6);
    }
}
