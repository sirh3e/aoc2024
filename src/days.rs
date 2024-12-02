use crate::parse::day01_parse;
use crate::{Day, DayResult, ResultValue};
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
