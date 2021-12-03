use anyhow::{Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let sonar = parse(input)?;
    let increases = count_increases(sonar.iter().cloned());
    Ok(format!("{:?}", increases))
}

pub fn part2(input: &str) -> Result<String> {
    let sonar = parse(input)?;
    let increases = count_increases(window_sums(&sonar, 3));
    Ok(format!("{:?}", increases))
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.parse::<i64>()
                .with_context(|| format!("could not parse number: {:?}", l))
        })
        .collect()
}

fn count_increases(mut depths: impl Iterator<Item = i64>) -> u64 {
    let mut prev = if let Some(first) = depths.next() {
        first
    } else {
        return 0;
    };
    let mut count = 0;
    for d in depths {
        if d > prev {
            count += 1;
        }
        prev = d;
    }
    count
}

fn window_sums(data: &[i64], window: usize) -> impl Iterator<Item = i64> + '_ {
    data.windows(window).map(|x| x.iter().sum::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day1");
    const EX: &str = include_str!("../../../input/day1_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "1451")
    }

    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "1395")
    }

    #[test]
    fn verify_example() {
        assert_eq!(part1(EX).unwrap().as_str(), "7")
    }

    #[test]
    fn verify_example_windows() {
        let ex_data = parse(EX).unwrap();
        let windowed = window_sums(&ex_data, 3).collect::<Vec<_>>();
        assert_eq!(windowed, vec![607, 618, 618, 617, 647, 716, 769, 792,])
    }
}
