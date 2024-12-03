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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Direction {
    Up(u32),
    Down(u32),
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum State {
    Valid,
    Invalid,
}

pub struct Day02;

impl Day02 {
    fn count(line: &Vec<u32>) -> usize {
        fn get_direction(a: u32, b: u32) -> Direction {
            let diff = a.abs_diff(b);
            match a < b {
                true => Direction::Up(diff),
                false => Direction::Down(diff),
            }
        }

        fn is_in_range(a: u32) -> bool {
            (1..=3).contains(&a)
        }

        let directions = line
            .iter()
            .zip(line.iter().skip(1))
            .map(|(a, b)| get_direction(*a, *b))
            .collect::<Vec<_>>();

        directions
            .iter()
            .zip(directions.iter().skip(1))
            .inspect(|(a, b)| println!("{:?} {:?}", a, b))
            .map(|(a, b)| match (a, b) {
                (Direction::Up(num1), Direction::Up(num2))
                | (Direction::Down(num1), Direction::Down(num2)) => {
                    is_in_range(*num1) && is_in_range(*num2)
                }
                _ => false,
            })
            .map(|b| match b {
                true => State::Valid,
                false => State::Invalid,
            })
            .filter(|state| *state == State::Invalid)
            .count()
    }
}

impl Day for Day02 {
    fn day(&self) -> u32 {
        2
    }

    fn part1(&self) -> DayResult {
        let lines = day2_parse("data/day02_part1.txt")?;
        let result = lines.iter().filter(|line| Self::count(line) == 0).count();

        Ok(ResultValue::from(result.to_string()))
    }

    fn part2(&self) -> DayResult {
        let lines = day2_parse("data/day02_part1.txt")?;
        let result = lines
            .iter()
            .filter_map(|line| {
                if Self::count(line) == 0 {
                    return Some(true);
                }

                for idx in 0..line.len() {
                    let mut part = line.clone();
                    part.remove(idx);

                    if Self::count(&part) == 0 {
                        return Some(true);
                    }
                }
                None
            })
            .count();

        Ok(ResultValue::from(result.to_string()))
    }
}
