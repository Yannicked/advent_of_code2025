use advent_of_code_2025::utils::read_grid;

fn is_roll(grid: &[Vec<char>], x: isize, y: isize) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    if let Some(line) = grid.get(y as usize) {
        if let Some(point) = line.get(x as usize) {
            return *point == '@';
        }
    }
    false
}

fn check_surrounding(grid: &[Vec<char>], x: isize, y: isize) -> i64 {
    let mut sum = 0;
    for xx in -1..=1 {
        for yy in -1..=1 {
            if xx == 0 && yy == 0 {
                continue;
            }
            if is_roll(grid, x + xx, y + yy) {
                sum += 1;
            }
        }
    }
    sum
}

fn count_rolls(grid: &[Vec<char>]) -> i64 {
    let mut sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '.' {
                continue;
            }
            let surroundings = check_surrounding(grid, x as isize, y as isize);
            if surroundings < 4 {
                sum += 1;
            }
        }
    }
    sum
}

fn count_rolls_edit(grid: &mut Vec<Vec<char>>) -> i64 {
    let mut sum = 0;
    let height = grid.len();
    let width = if height > 0 { grid[0].len() } else { 0 };

    let mut to_change = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != '@' {
                continue;
            }

            let surroundings = check_surrounding(grid, x as isize, y as isize);

            if surroundings < 4 {
                to_change.push((x, y, surroundings));
                sum += 1;
            }
        }
    }

    for (x, y, surroundings) in to_change {
        grid[y][x] = 'x';
    }
    sum
}

fn part1(input: &str) -> i64 {
    let grid = read_grid(input);
    count_rolls(&grid)
}

fn part2(input: &str) -> i64 {
    let mut grid = read_grid(input);
    let mut sum = 0;
    loop {
        let new_rolls = count_rolls_edit(&mut grid);
        if new_rolls == 0 {
            break;
        }
        sum += new_rolls;
    }
    sum
}

fn main() {
    let input = include_str!("../../inputs/day04.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../inputs/day04_test.txt");
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day04_test.txt");
        assert_eq!(part2(input), 43);
    }
}
