advent_of_code::solution!(4);
use anyhow::Result;
use md5::{Digest, Md5};

pub fn part_one(input: &str) -> Option<u32> {
    match find_hash_end(input, "00000") {
        Ok(result) => Some(result),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    match find_hash_end(input, "000000") {
        Ok(result) => Some(result),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}

fn find_hash_end(input: &str, prefix: &str) -> Result<u32> {
    let mut hasher = Md5::new();
    for i in 0.. {
        hasher.update(format!("{}{}", input.trim(), i));
        let result = format!("{:x}", hasher.finalize_reset());
        if result.starts_with(prefix) {
            return Ok(i);
        }

        if i == u32::MAX {
            return Err(anyhow::anyhow!("No hash found"));
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6742839));
    }
}
