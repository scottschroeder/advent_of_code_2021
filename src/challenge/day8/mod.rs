use anyhow::{Context, Result};
use sevensegment::SevenSegment;
mod sevensegment;

pub fn part1(input: &str) -> Result<String> {
    let observations = parse(input)?;
    let unqiue = observations
        .iter()
        .flat_map(|(i, o)| o.iter())
        .filter_map(|s| sevensegment::guess_digit(*s))
        .count();
    Ok(format!("{:?}", unqiue))
}

pub fn part2(input: &str) -> Result<String> {
    let x = 0;
    Ok(format!("{:?}", x))
}

fn parse(input: &str) -> Result<Vec<(Vec<SevenSegment>, Vec<SevenSegment>)>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> Result<(Vec<SevenSegment>, Vec<SevenSegment>)> {
    let mut chunks = input.split('|');
    let digits_chunk = chunks
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not parse: {:?}", input))?;
    let numbers_chunk = chunks
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not parse: {:?}", input))?;
    let digits = parse_displays(digits_chunk)?;
    let numbers = parse_displays(numbers_chunk)?;
    Ok((digits, numbers))
}

fn parse_displays(input: &str) -> Result<Vec<SevenSegment>> {
    input
        .split_whitespace()
        .map(|l| {
            SevenSegment::from_wire_code(l)
                .with_context(|| format!("could not parse segment: {:?}", l))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day8");
    const EX: &str = include_str!("../../../input/day8_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "303")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "0")
    }
    #[test]
    fn check_p1_example() {
        assert_eq!(part1(EX).unwrap().as_str(), "26")
    }
    #[test]
    fn check_p2_example() {
        assert_eq!(part2(EX).unwrap().as_str(), "0")
    }
}
