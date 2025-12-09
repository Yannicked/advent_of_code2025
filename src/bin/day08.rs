use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Coordinate {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<i64> = s
            .split(',')
            .map(|n| n.trim().parse())
            .collect::<Result<_, _>>()?;

        Ok(Coordinate {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        })
    }
}

impl Coordinate {
    fn distance_squared(&self, other: &Coordinate) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx.pow(2) + dy.pow(2) + dz.pow(2)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Edge {
    p1_index: usize,
    p2_index: usize,
    dist_sq: i64,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist_sq.cmp(&other.dist_sq)
    }
}

fn parse_coordinates(input: &str) -> Vec<Coordinate> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn generate_sorted_edges(coords: &[Coordinate]) -> Vec<Edge> {
    let mut edges = Vec::with_capacity(coords.len().pow(2));
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            edges.push(Edge {
                p1_index: i,
                p2_index: j,
                dist_sq: coords[i].distance_squared(&coords[j]),
            });
        }
    }
    edges.sort_unstable();
    edges
}

fn part1(input: &str, x: usize) -> usize {
    let coords = parse_coordinates(input);

    let edges = generate_sorted_edges(&coords);

    let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();

    for edge in edges.iter().take(x) {
        adjacency_list
            .entry(edge.p1_index)
            .or_default()
            .push(edge.p2_index);
        adjacency_list
            .entry(edge.p2_index)
            .or_default()
            .push(edge.p1_index);
    }

    let mut visited: HashSet<usize> = HashSet::new();
    let mut component_sizes: Vec<usize> = Vec::new();

    for start_node in 0..coords.len() {
        if visited.contains(&start_node) {
            continue;
        }

        let mut current_component_size = 0;
        let mut queue = vec![start_node];
        visited.insert(start_node);

        while let Some(node) = queue.pop() {
            current_component_size += 1;

            if let Some(neighbors) = adjacency_list.get(&node) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push(neighbor);
                    }
                }
            }
        }
        component_sizes.push(current_component_size);
    }

    component_sizes.sort_unstable_by(|a, b| b.cmp(a));
    component_sizes.iter().take(3).product()
}

fn part2(input: &str) -> i64 {
    let coords = parse_coordinates(input);
    let edges = generate_sorted_edges(&coords);

    let num_points = coords.len();
    let mut group_ids: Vec<usize> = (0..num_points).collect();
    let mut clusters_remaining = num_points;

    for edge in edges {
        let color_1 = group_ids[edge.p1_index];
        let color_2 = group_ids[edge.p2_index];

        if color_1 != color_2 {
            for id in group_ids.iter_mut() {
                if *id == color_2 {
                    *id = color_1;
                }
            }

            clusters_remaining -= 1;

            if clusters_remaining == 1 {
                return coords[edge.p1_index].x * coords[edge.p2_index].x;
            }
        }
    }
    0
}

fn main() {
    let input = include_str!("../../inputs/day08.txt");
    println!("Part 1: {}", part1(input, 1000));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../inputs/day08_test.txt");
        assert_eq!(part1(input, 10), 40);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../inputs/day08_test.txt");
        assert_eq!(part2(input), 25272);
    }
}
