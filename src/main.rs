use aoc2024::days::{Day01, Day02};
use aoc2024::Day;

fn main() -> aoc2024::Result<()> {
    //let days = vec![Box::new(Day01) as Box<dyn Day>];
    let days = vec![Box::new(Day02) as Box<dyn Day>];
    for day in days {
        println!("{:?}", day.output()?);
    }

    Ok(())
}
