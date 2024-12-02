use std::{collections::HashMap, path::Path};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct ResultValue(String);

impl From<String> for ResultValue {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<u32> for ResultValue {
    fn from(value: u32) -> Self {
        Self::from(value.to_string())
    }
}

type DayResult = Result<ResultValue>;

trait Day {
    fn day(&self) -> u32;
    fn part1(&self) -> DayResult;
    fn part2(&self) -> DayResult;

    fn output(&self) -> DayResult {
        let output = format!(
            r#"Day: {}
Part 1: {:?},
Part 2: {:?}"#,
            self.day(),
            self.part1(),
            self.part2()
        );
        Ok(ResultValue::from(output))
    }
}

struct Day01;

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

fn main() -> Result<()> {
    let days = vec![Box::new(Day01) as Box<dyn Day>];
    for day in days {
        println!("{:?}", day.output()?);
    }

    Ok(())
}

fn day01_parse<P>(path: P) -> Result<(Vec<u32>, Vec<u32>)>
where
    P: AsRef<Path>,
{
    let content = std::fs::read_to_string(path)?;

    let (lhs, rhs): (Vec<_>, Vec<_>) = content
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<_>>();

            let num1 = parts[0].parse::<u32>().unwrap();
            let num2 = parts[parts.len() - 1].parse::<u32>().unwrap();
            (num1, num2)
        })
        .unzip();

    Ok((lhs, rhs))
}
