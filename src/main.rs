use aoc2024::{
    days::{day01::Day01, day02::Day02},
    Day, Result,
};

fn main() -> Result<()> {
    let days = vec![
        Box::new(Day01) as Box<dyn Day>,
        Box::new(Day02) as Box<dyn Day>,
    ];
    for day in days {
        println!("{:?}", day.output()?);
    }

    Ok(())
}
