use anyhow::{Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let fuel = find_min_fuel_transfer(input, linear_error)?;
    Ok(format!("{:?}", fuel))
}

pub fn part2(input: &str) -> Result<String> {
    let fuel = find_min_fuel_transfer(input, step_error)?;
    Ok(format!("{:?}", fuel))
}

fn find_min_fuel_transfer<F>(input: &str, err_func: F) -> Result<i64>
where
    F: Fn(i64, i64) -> i64,
{
    let crabs = parse(input)?;
    let (min, max) = minmax(&crabs)?;
    let (target, fuel) = (min..max)
        .enumerate()
        .map(|(idx, t)| (idx, total_error(&crabs, t, &err_func)))
        .min_by_key(|(_, e)| *e)
        .ok_or_else(|| anyhow::anyhow!("no possible targets"))?;
    log::debug!("target: {:?}, fuel: {:?}", target, fuel);
    Ok(fuel)
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .trim()
        .split(',')
        .map(|l| {
            l.parse::<i64>()
                .with_context(|| format!("could not parse number: {:?}", l))
        })
        .collect()
}

fn total_error<F>(data: &[i64], target: i64, err_func: F) -> i64
where
    F: Fn(i64, i64) -> i64,
{
    data.iter().map(|d| err_func(target, *d).abs()).sum()
}

fn linear_error(target: i64, source: i64) -> i64 {
    (target - source).abs()
}

fn step_error(target: i64, source: i64) -> i64 {
    let distance = (target - source).abs();
    arithmetic_progression(distance)
}

fn arithmetic_progression(n: i64) -> i64 {
    (n * n + n) / 2
}

fn minmax(data: &[i64]) -> Result<(i64, i64)> {
    if data.is_empty() {
        anyhow::bail!("can not get min/max from empty list");
    }
    let mut min = data[0];
    let mut max = data[0];

    for i in data.iter().skip(1).cloned() {
        if i < min {
            min = i
        }
        if i > max {
            max = i
        }
    }

    Ok((min, max))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day7");
    const EX: &str = include_str!("../../../input/day7_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "344735")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "96798233")
    }

    #[test]
    fn check_example_p1() {
        assert_eq!(part1(EX).unwrap().as_str(), "37")
    }
    #[test]
    fn check_example_p2() {
        assert_eq!(part2(EX).unwrap().as_str(), "168")
    }

    #[test]
    fn check_newton_sum() {
        assert_eq!(arithmetic_progression(100), 5050)
    }
}
