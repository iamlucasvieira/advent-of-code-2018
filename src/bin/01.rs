advent_of_code::solution!(1);

/// floor_change returns the change in floor level for a given character.
fn floor_change(c: char) -> Option<i32> {
    match c {
        '(' => Some(1),
        ')' => Some(-1),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    input
        .trim()
        .chars()
        .try_fold(0i32, |acc, c| floor_change(c).map(|change| acc + change))
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .trim()
        .chars()
        .enumerate()
        .scan(0i32, |floor, (i, c)| {
            *floor += match floor_change(c) {
                Some(new_floor) => new_floor,
                None => return None,
            };
            Some((i + 1, *floor))
        })
        .find_map(|(i, floor)| if floor == -1 { Some(i) } else { None })
        .map(|i| i as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(-3));
    }

    #[test]
    fn test_part_one_invalid() {
        let result = part_one("1");
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_one_negative() {
        let result = part_one("))");
        assert_eq!(result, Some(-2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_invalid() {
        let result = part_two("1");
        assert_eq!(result, None);
    }
}
