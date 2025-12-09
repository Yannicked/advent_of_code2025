fn is_periodic(s: &str) -> bool {
    // Check if the pattern is periodic by:
    // repeating the string (123123) -> (123123123123)
    // removing the first and last characters -> (2312312312)
    // Checking if the original pattern is still present
    s.len() > 1 && format!("{}{}", s, s)[1..s.len() * 2 - 1].contains(s)
}

fn is_twice(s: &str) -> bool {
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }

    let pattern1 = &s[0..len / 2];
    let pattern2 = &s[len / 2..len];
    return pattern1 == pattern2;
}

fn part1(input: &str) -> i64 {
    let parts = input.split(",");

    let mut sum: i64 = 0;
    for part in parts {
        if let Some((part1, part2)) = part.split_once('-') {
            let num1: i64 = part1.trim().parse().expect("Not a number");
            let num2: i64 = part2.trim().parse().expect("Not a number");

            for num in num1..=num2 {
                if is_twice(&num.to_string()) {
                    // println!("Invalid: {}", num);
                    sum += num;
                }
            }
        }
    }

    return sum;
}

fn part2(input: &str) -> i64 {
    let parts = input.split(",");

    let mut sum: i64 = 0;
    for part in parts {
        if let Some((part1, part2)) = part.split_once('-') {
            let num1: i64 = part1.trim().parse().expect("Not a number");
            let num2: i64 = part2.trim().parse().expect("Not a number");

            for num in num1..=num2 {
                if is_periodic(&num.to_string()) {
                    // println!("Invalid: {}", num);
                    sum += num;
                }
            }
        }
    }

    return sum;
}

fn main() {
    let input = include_str!("../../inputs/day02.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../inputs/day02_test.txt");
        assert_eq!(part1(input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day02_test.txt");
        assert_eq!(part2(input), 4174379265);
    }
}
