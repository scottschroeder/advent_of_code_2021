use anyhow::{Context, Result};

mod bingo;

pub fn part1(input: &str) -> Result<String> {
    let w = iterate_bingo_winners(input)?
        .next()
        .ok_or_else(|| anyhow::anyhow!("no winner"))?;

    Ok(format!("{}", w))
}

pub fn part2(input: &str) -> Result<String> {
    let w = iterate_bingo_winners(input)?
        .last()
        .ok_or_else(|| anyhow::anyhow!("no winner"))?;

    Ok(format!("{}", w))
}

fn iterate_bingo_winners(input: &str) -> Result<impl Iterator<Item = u64>> {
    let (called, cards) = parse(input)?;

    let mut mb = bingo::MultiBingo::default();

    for card in cards.chunks_exact(bingo::BINGO_LEN) {
        mb.add_card(card);
    }
    Ok(mb.iter(called.into_iter()))
}

fn parse(input: &str) -> Result<(Vec<u8>, Vec<u8>)> {
    let mut lines = input.lines();
    let first_line = lines
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not get first line"))?;

    let called = first_line
        .split(',')
        .map(|num| {
            num.parse::<u8>()
                .with_context(|| format!("could not parse number: {:?}", num))
        })
        .collect::<Result<Vec<_>>>()?;

    let cards = lines
        .flat_map(|l| l.split_whitespace())
        .map(|num| {
            num.parse::<u8>()
                .with_context(|| format!("could not parse number: {:?}", num))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((called, cards))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day4");
    const EX: &str = include_str!("../../../input/day4_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "35711")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "5586")
    }
    #[test]
    fn check_p1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "4512")
    }
    #[test]
    fn check_p2_ex() {
        assert_eq!(part2(EX).unwrap().as_str(), "1924")
    }
}
