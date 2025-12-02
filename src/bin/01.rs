advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().ok()?;

        position = match direction {
            "L" => (position + 100 - distance % 100) % 100,
            "R" => (position + distance) % 100,
            _ => return None,
        };

        if position == 0 {
            zero_count += 1;
        }
    }

    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().ok()?;

        let zeros_during_rotation = match direction {
            "L" => {
                let first_zero_at = if position == 0 { 100 } else { position };
                if distance >= first_zero_at {
                    1 + (distance - first_zero_at) / 100
                } else {
                    0
                }
            }
            "R" => {
                let first_zero_at = if position == 0 { 100 } else { 100 - position };
                if distance >= first_zero_at {
                    1 + (distance - first_zero_at) / 100
                } else {
                    0
                }
            }
            _ => return None,
        };

        zero_count += zeros_during_rotation;

        position = match direction {
            "L" => (position + 100 - distance % 100) % 100,
            "R" => (position + distance) % 100,
            _ => return None,
        };
    }

    Some(zero_count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
