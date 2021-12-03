use anyhow::{Context, Result};
use std::str::FromStr;

pub fn part1(input: &str) -> Result<String> {
    let commands = parse(input)?;
    let mut loc = Location::default();
    for c in commands {
        loc.update(&c)
    }
    log::trace!("final location: {:?}", loc);
    Ok(format!("{}", loc.x * loc.depth))
}

pub fn part2(input: &str) -> Result<String> {
    let commands = parse(input)?;
    let mut loc = Location::default();
    for c in commands {
        loc.update_with_aim(&c)
    }
    log::trace!("final location: {:?}", loc);
    Ok(format!("{}", loc.x * loc.depth))
}

fn parse(input: &str) -> Result<Vec<Command>> {
    input.lines().map(|l| l.parse::<Command>()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Forward,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => anyhow::bail!("unable to parse direction: {:?}", s),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Command {
    direction: Direction,
    units: u32,
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let (dir, mag) = split
            .next()
            .and_then(|first| split.next().map(|second| (first, second)))
            .ok_or_else(|| anyhow::anyhow!("unable to parse command from {:?}", s))?;
        let units = mag
            .parse::<u32>()
            .with_context(|| format!("unable to parse number: {:?}", mag))?;
        let direction = dir.parse::<Direction>()?;
        Ok(Command { direction, units })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Location {
    x: i32,
    aim: i32,
    depth: i32,
}

impl Location {
    fn update(&mut self, cmd: &Command) {
        let delta = cmd.units as i32;
        match cmd.direction {
            Direction::Up => self.depth -= delta,
            Direction::Down => self.depth += delta,
            Direction::Forward => self.x += delta,
        }
    }
    fn update_with_aim(&mut self, cmd: &Command) {
        let delta = cmd.units as i32;
        match cmd.direction {
            Direction::Up => self.aim -= delta,
            Direction::Down => self.aim += delta,
            Direction::Forward => {
                self.x += delta;
                self.depth += delta * self.aim;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day2");
    const EX: &str = include_str!("../../../input/day2_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "1648020")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "1759818555")
    }
    #[test]
    fn check_example() {
        assert_eq!(part1(EX).unwrap().as_str(), "150")
    }
    #[test]
    fn check_example_pt2() {
        assert_eq!(part2(EX).unwrap().as_str(), "900")
    }
}
