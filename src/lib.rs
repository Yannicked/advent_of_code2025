pub mod utils {
    // Placeholder for shared utilities
    pub fn read_lines(input: &str) -> Vec<&str> {
        input.lines().collect()
    }

    pub fn read_grid(input: &str) -> Vec<Vec<char>> {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line: &str| line.chars().collect())
            .collect();
        return grid;
    }
}
