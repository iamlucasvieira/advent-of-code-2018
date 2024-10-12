advent_of_code::solution!(3);
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::ops::Add;
use std::ops::AddAssign;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn from_direction(direction: char) -> Result<Self> {
        match direction {
            '^' => Ok(Self::new(0, 1)),
            'v' => Ok(Self::new(0, -1)),
            '>' => Ok(Self::new(1, 0)),
            '<' => Ok(Self::new(-1, 0)),
            _ => Err(anyhow::anyhow!("Invalid direction")),
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

type Visited = HashSet<Point>;

fn parse_coordinates(input: &str) -> Result<Visited> {
    let mut santa = Point::new(0, 0);
    let mut visited = Visited::new();

    visited.insert(santa);

    input
        .trim()
        .chars()
        .try_for_each(|c| -> Result<()> {
            let delta = Point::from_direction(c).with_context(|| "Invalid direction")?;
            santa += delta;
            visited.insert(santa);
            Ok(())
        })
        .with_context(|| "Error parsing coordinates")?;

    Ok(visited)
}

fn parse_coordinates_with_robot(input: &str) -> Result<Visited> {
    let mut visited = Visited::new();

    // Get input skipping every other character
    let santa_input = input.trim().chars().step_by(2).collect::<String>();
    let robot_input = input.trim().chars().skip(1).step_by(2).collect::<String>();

    let santa_visited = parse_coordinates(&santa_input)?;
    let robot_visited = parse_coordinates(&robot_input)?;

    visited.extend(santa_visited);
    visited.extend(robot_visited);

    Ok(visited)
}

pub fn part_one(input: &str) -> Option<u32> {
    match parse_coordinates(input) {
        Ok(visited) => Some(visited.len() as u32),
        Err(_) => {
            eprintln!("Error parsing coordinates");
            None
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    match parse_coordinates_with_robot(input) {
        Ok(visited) => Some(visited.len() as u32),
        Err(_) => {
            eprintln!("Error parsing coordinates");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }
}
