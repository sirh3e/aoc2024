use crate::{
    parse::{day01_parse, day2_parse},
    Day, DayResult, ResultValue,
};
use std::collections::HashMap;

pub struct Day01;
impl Day for Day01 {
    fn day(&self) -> u32 {
        1
    }

    fn part1(&self) -> DayResult {
        let (mut lhs, mut rhs) = day01_parse("data/day01_part1.txt")?;
        lhs.sort();
        rhs.sort();

        let result = lhs
            .iter()
            .zip(rhs.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum::<u32>();

        Ok(ResultValue::from(format!("{}", result).to_string()))
    }

    fn part2(&self) -> DayResult {
        let (lhs, rhs) = day01_parse("data/day01_part2.txt")?;

        fn vec_into_hash_map(vec: Vec<u32>) -> HashMap<u32, u32> {
            vec.into_iter().fold(HashMap::new(), |mut acc, num| {
                *acc.entry(num).or_insert(0) += 1;
                acc
            })
        }

        let lhs_map = vec_into_hash_map(lhs);
        let rhs_map = vec_into_hash_map(rhs);

        let result: u32 = lhs_map
            .iter()
            .filter_map(|(key, amount)| rhs_map.get(key).map(|value| key * amount * value))
            .sum();

        Ok(ResultValue::from(format!("{}", result).to_string()))
    }
}

pub struct Day02;

impl Day for Day02 {
    fn day(&self) -> u32 {
        2
    }

    fn part1(&self) -> DayResult {
        #[derive(Eq, PartialEq, Copy, Clone)]
        enum Direction {
            Up,
            Down,
            Invalid,
            Unutilized,
        }

        fn valid(line: &Vec<u32>) -> bool {
            fn get_direction(a: u32, b: u32) -> Direction {
                match a < b {
                    true => Direction::Up,
                    false => Direction::Down,
                }
            }

            let (direction, _) = line.iter().skip(1).fold(
                (Direction::Unutilized, line[0]),
                |(direction, old), new| {
                    let new_direction = get_direction(*new, old);
                    let diff = new.abs_diff(old);

                    match (
                        direction,
                        direction == new_direction,
                        diff >= 1 && diff <= 3,
                    ) {
                        (Direction::Unutilized, _, true) => (new_direction, *new),
                        (_, true, true) => (new_direction, *new),
                        _ => return (Direction::Invalid, 0),
                    }
                },
            );

            !(Direction::Invalid == direction)
        }

        let lines = day2_parse("data/day02_part1.txt")?;
        let result = lines.iter().filter(|line| valid(line)).count();

        Ok(ResultValue::from(result.to_string()))
    }

    fn part2(&self) -> DayResult {
        Ok(ResultValue::from("Day 2 Part 2".to_string()))
    }
}
