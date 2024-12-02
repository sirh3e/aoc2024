use std::path::Path;

pub fn day01_parse<P>(path: P) -> crate::Result<(Vec<u32>, Vec<u32>)>
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
