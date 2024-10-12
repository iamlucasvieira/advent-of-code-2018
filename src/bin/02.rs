advent_of_code::solution!(2);
use anyhow::{Context, Result};

pub fn part_one(input: &str) -> Option<u32> {
    // log error and return None
    match get_total_paper_needed(input) {
        Ok(result) => Some(result),
        Err(err) => {
            println!("Error: {}", err);
            None
        }
    }
}

/// Parse a line of input into a sorted array of dimensions
fn parse_line(line: &str) -> Result<[u32; 3]> {
    let mut dims: [u32; 3] = line
        .trim()
        .split('x')
        .map(|dim| dim.parse())
        .collect::<Result<Vec<u32>, _>>()
        .with_context(|| format!("Invalid conversion: {}", line))?
        .try_into()
        .map_err(|_| anyhow::anyhow!("Invalid number of dimensions"))?;

    dims.sort_unstable();

    Ok(dims)
}

/// Calculate the total amount of paper needed for all the presents
fn get_total_paper_needed(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            let dims = parse_line(line)?;
            let [l, w, h] = dims;
            let surface_area = 2 * l * w + 2 * w * h + 2 * h * l;
            let slack = l * w;
            Ok(surface_area + slack)
        })
        .sum()
}

/// Calculate the total amount of ribbon needed for all the presents
fn get_total_ribbon_needed(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            let dims = parse_line(line)?;
            let [l, w, h] = dims;
            let ribbon = 2 * l + 2 * w;
            let bow = l * w * h;
            Ok(ribbon + bow)
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    match get_total_ribbon_needed(input) {
        Ok(result) => Some(result),
        Err(err) => {
            println!("Error: {}", err);
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
        assert_eq!(result, Some(101));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
