pub mod days;
mod parse;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct ResultValue(String);

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

pub type DayResult = Result<ResultValue>;

pub trait Day {
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
