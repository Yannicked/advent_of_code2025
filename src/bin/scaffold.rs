use std::env;
use std::fs;
use std::path::Path;
use std::process;

const TEMPLATE: &str = r#"fn part1(input: &str) -> String {
    "todo".to_string()
}

fn part2(input: &str) -> String {
    "todo".to_string()
}

fn main() {
    let input = include_str!("../../inputs/dayDAY.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../inputs/dayDAY_test.txt");
        assert_eq!(part1(input), "todo");
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/dayDAY_test.txt");
        assert_eq!(part2(input), "todo");
    }
}
"#;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin scaffold <day>");
        eprintln!("Example: cargo run --bin scaffold 1");
        process::exit(1);
    }

    let day: u32 = match args[1].parse() {
        Ok(d) if d >= 1 && d <= 25 => d,
        _ => {
            eprintln!("Day must be between 1 and 25");
            process::exit(1);
        }
    };

    let day_padded = format!("{:02}", day);
    let module_name = format!("day{}", day_padded);
    let input_filename = format!("day{}.txt", day_padded);
    let test_input_filename = format!("day{}_test.txt", day_padded);

    let rs_path = Path::new("src")
        .join("bin")
        .join(format!("{}.rs", module_name));
    let input_path = Path::new("inputs").join(&input_filename);
    let test_input_path = Path::new("inputs").join(&test_input_filename);

    // Create input file if it doesn't exist
    if !input_path.exists() {
        match fs::write(&input_path, "") {
            Ok(_) => println!("Created empty input file: {}", input_path.display()),
            Err(e) => eprintln!("Failed to create input file: {}", e),
        }
    } else {
        println!("Input file already exists: {}", input_path.display());
    }
    if !test_input_path.exists() {
        match fs::write(&test_input_path, "") {
            Ok(_) => println!(
                "Created empty test input file: {}",
                test_input_path.display()
            ),
            Err(e) => eprintln!("Failed to create test input file: {}", e),
        }
    } else {
        println!(
            "Test input file already exists: {}",
            test_input_path.display()
        );
    }

    // Create Rust file if it doesn't exist
    if rs_path.exists() {
        eprintln!(
            "File {} already exists. Skipping generation.",
            rs_path.display()
        );
        return;
    }

    let code = TEMPLATE.replace("dayDAY", &module_name);

    match fs::write(&rs_path, code) {
        Ok(_) => println!("Created rust file: {}", rs_path.display()),
        Err(e) => eprintln!("Failed to create rust file: {}", e),
    }

    println!("Done! You can run the day with:");
    println!("cargo run --bin {}", module_name);
}
