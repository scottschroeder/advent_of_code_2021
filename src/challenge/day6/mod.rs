use anyhow::{Context, Result};

const FISH_BREED_RATE: usize = 6;
const FISH_INCUBATION_RATE: usize = 8;
const FISH_LEN: usize = FISH_INCUBATION_RATE + 1;
const PART_1_FISH_TIME: usize = 80;
const PART_2_FISH_TIME: usize = 256;

pub fn part1(input: &str) -> Result<String> {
    let fish = parse(input)?;
    let mut colony = FishColony::new(&fish);
    colony.fish_days(PART_1_FISH_TIME);
    Ok(format!("{:?}", colony.total()))
}

pub fn part2(input: &str) -> Result<String> {
    let fish = parse(input)?;
    let mut colony = FishColony::new(&fish);
    colony.fish_days(PART_2_FISH_TIME);
    Ok(format!("{:?}", colony.total()))
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

#[derive(Debug, Default)]
struct FishColony {
    fish: [usize; FISH_LEN],
}

impl FishColony {
    fn new(starting_fish: &[i64]) -> FishColony {
        let mut colony = FishColony::default();
        for f in starting_fish {
            colony.fish[*f as usize] += 1;
        }
        colony
    }
    fn fish_day_step(&mut self) {
        let mut new_fish = [0usize; FISH_LEN];
        for (idx, c) in self.fish.into_iter().enumerate() {
            if idx == 0 {
                new_fish[FISH_BREED_RATE] = c;
                new_fish[FISH_INCUBATION_RATE] = c;
            } else {
                new_fish[idx - 1] += c;
            }
        }
        self.fish = new_fish
    }
    fn fish_days(&mut self, days: usize) {
        log::trace!("Initial State: {:?}", self);
        for d in 0..days {
            self.fish_day_step();
            log::trace!("Day {}: {:?}", d, self);
        }
    }
    fn total(&self) -> usize {
        self.fish.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day6");
    const EX: &str = include_str!("../../../input/day6_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "385391")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "1728611055389")
    }

    #[test]
    fn check_example_p1() {
        assert_eq!(part1(EX).unwrap().as_str(), "5934")
    }
    #[test]
    fn check_example_p2() {
        assert_eq!(part2(EX).unwrap().as_str(), "26984457539")
    }
}
