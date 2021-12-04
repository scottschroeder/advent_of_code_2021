use anyhow::{Context, Result};
use std::fmt;

pub fn part1(input: &str) -> Result<String> {
    let diag = parse(input)?;
    let max_bits = get_max_bits_for_diag(&diag);
    let g = gamma(&diag, max_bits);
    let e = epsilon(&diag, max_bits);
    log::debug!("gamma: {:0b} epsilon: {:0b}", g, e,);
    Ok(format!("{:?}", g * e))
}

pub fn part2(input: &str) -> Result<String> {
    let diag = parse(input)?;
    let max_bits = get_max_bits_for_diag(&diag);
    let o2 = oxygen(&diag, max_bits);
    let co2 = carbon_dioxide(&diag, max_bits);
    log::debug!("oxygen: {}, carbon dioxide: {}", o2, co2);
    Ok(format!("{:?}", o2 * co2))
}

fn parse(input: &str) -> Result<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            u32::from_str_radix(l, 2).with_context(|| format!("could not parse number: {:?}", l))
        })
        .collect()
}

fn gamma(diag: &[u32], max_bits: u32) -> u32 {
    get_all_bits_by(get_most_common_bit, diag, max_bits)
}

fn epsilon(diag: &[u32], max_bits: u32) -> u32 {
    get_all_bits_by(get_least_common_bit, diag, max_bits)
}

fn oxygen(diag: &[u32], max_bits: u32) -> u32 {
    find_by_reduction(get_most_common_bit, diag, max_bits)
}

fn carbon_dioxide(diag: &[u32], max_bits: u32) -> u32 {
    find_by_reduction(get_least_common_bit, diag, max_bits)
}

fn get_max_bits_for_diag(data: &[u32]) -> u32 {
    let mut all_bits = 0;
    for d in data {
        all_bits |= d;
    }
    u32::BITS - all_bits.leading_zeros()
}

#[inline]
fn is_bit_high(x: u32, bit: u32) -> bool {
    x & 1u32 << bit > 0
}

fn is_most_common_bit_high(data: &[u32], idx: u32) -> bool {
    let total = data.iter().filter(|l| is_bit_high(**l, idx)).count();
    total * 2 >= data.len()
}

fn get_most_common_bit(data: &[u32], idx: u32) -> u32 {
    if is_most_common_bit_high(data, idx) {
        1u32 << idx
    } else {
        0
    }
}

fn get_least_common_bit(data: &[u32], idx: u32) -> u32 {
    if is_most_common_bit_high(data, idx) {
        0
    } else {
        1u32 << idx
    }
}

fn get_all_bits_by<F>(reducer: F, data: &[u32], max_bits: u32) -> u32
where
    F: Fn(&[u32], u32) -> u32,
{
    let mut result = 0;
    for b in 0..max_bits {
        result |= reducer(data, b);
    }
    result
}

fn reduce_bitwise_by<F>(reducer: F, data: &mut Vec<u32>, idx: u32)
where
    F: Fn(&[u32], u32) -> u32,
{
    let filter = reducer(data.as_slice(), idx);
    let mask = 1u32 << idx;
    let check = |x: &u32| {
        let r = x & mask == filter;
        log::trace!(
            "(b={}) {:08b} & {:08b} == {:08b} => {:?}",
            idx,
            x,
            mask,
            filter,
            r
        );
        r
    };
    data.retain(check)
}

fn find_by_reduction<F>(reducer: F, data: &[u32], max_bits: u32) -> u32
where
    F: Fn(&[u32], u32) -> u32,
{
    let mut matches = data.to_owned();
    for b in 0..max_bits {
        log::trace!("{:#?}", BinList(matches.as_slice(), max_bits as usize));
        reduce_bitwise_by(&reducer, &mut matches, max_bits - b - 1);
        if matches.len() == 1 {
            break;
        }
    }
    log::trace!(
        "FINAL {:#?}",
        BinList(matches.as_slice(), max_bits as usize)
    );
    matches[0]
}

struct BinList<'a>(&'a [u32], usize);

impl<'a> fmt::Debug for BinList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dl = f.debug_list();
        for x in self.0 {
            dl.entry(&format_args!("{:0width$b}", x, width = self.1));
        }
        dl.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day3");
    const EX: &str = include_str!("../../../input/day3_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "738234")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "3969126")
    }
    #[test]
    fn verify_example() {
        assert_eq!(part1(EX).unwrap().as_str(), "198")
    }
    #[test]
    fn verify_example_p2() {
        assert_eq!(part2(EX).unwrap().as_str(), "230")
    }

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn bit_high_check() {
        assert_eq!(is_bit_high(0b0001, 0), true);
        assert_eq!(is_bit_high(0b0001, 1), false);
        assert_eq!(is_bit_high(0b0011, 0), true);
        assert_eq!(is_bit_high(0b0011, 1), true);
        assert_eq!(is_bit_high(0b0010, 0), false);
        assert_eq!(is_bit_high(0b0010, 1), true);
    }
    #[test]
    fn check_max_bits() {
        let data = &[0b0001, 0b0100];
        assert_eq!(get_max_bits_for_diag(data), 3);
    }
}
