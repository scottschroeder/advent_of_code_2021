use anyhow::{Context, Result};
use aoc::Point;
use std::collections::HashMap;

pub fn part1(input: &str) -> Result<String> {
    let overlaps = count_intersections(input, true)?;
    Ok(format!("{:?}", overlaps))
}

pub fn part2(input: &str) -> Result<String> {
    let overlaps = count_intersections(input, false)?;
    Ok(format!("{:?}", overlaps))
}

fn count_intersections(input: &str, filter_straight: bool) -> Result<usize> {
    let segments = parse(input)?;
    let mut hitmap: HashMap<Point, u32> = HashMap::new();
    let hit_inc = |p: Point| {
        let e = hitmap.entry(p).or_default();
        *e += 1;
    };

    segments
        .iter()
        .filter(|l| l.line_type().filter_straight(filter_straight))
        .flat_map(|l| l.all_points())
        .for_each(hit_inc);

    Ok(hitmap.values().filter(|c| **c >= 2).count())
}

fn parse(input: &str) -> Result<Vec<Line>> {
    input.lines().map(parse_line).collect()
}

fn parse_line(input: &str) -> Result<Line> {
    let mut segments = input.split(" -> ");
    let start_str = segments
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not parse points: {:?}", input))?;
    let end_str = segments
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not parse points: {:?}", input))?;
    let start = parse_point(start_str)?;
    let end = parse_point(end_str)?;
    Ok(Line { start, end })
}

fn parse_point(input: &str) -> Result<Point> {
    let mut segments = input.split(',');
    let x_str = segments
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not parse point: {:?}", input))?;
    let y_str = segments
        .next()
        .ok_or_else(|| anyhow::anyhow!("could not parse point: {:?}", input))?;
    let x = x_str
        .parse::<i64>()
        .with_context(|| format!("could not parse number: {:?}", x_str))?;
    let y = y_str
        .parse::<i64>()
        .with_context(|| format!("could not parse number: {:?}", y_str))?;
    Ok(Point::new(x, y))
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LineType {
    Horizontal,
    Vertical,
    Other,
}

impl LineType {
    fn is_stright(self) -> bool {
        match self {
            LineType::Horizontal => true,
            LineType::Vertical => true,
            LineType::Other => false,
        }
    }
    fn filter_straight(self, do_filter: bool) -> bool {
        !do_filter || self.is_stright()
    }
}

#[derive(Debug, Clone)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn line_type(&self) -> LineType {
        if self.start.x == self.end.x {
            LineType::Vertical
        } else if self.start.y == self.end.y {
            LineType::Horizontal
        } else {
            LineType::Other
        }
    }

    fn all_points(&self) -> LinePoints {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        // Slopes will never be rationals, just +/- 0,1 & 1,0 & 1,1
        let slope = Point::new(
            if dx == 0 { 0 } else { dx / dx.abs() },
            if dy == 0 { 0 } else { dy / dy.abs() },
        );
        LinePoints {
            current: self.start,
            target: self.end + slope,
            slope,
        }
    }
}

#[derive(Debug)]
struct LinePoints {
    current: Point,
    target: Point,
    slope: Point,
}

impl Iterator for LinePoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.target {
            None
        } else {
            let ret = self.current;
            self.current += self.slope;
            Some(ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day5");
    const EX: &str = include_str!("../../../input/day5_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "6572")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "21466")
    }

    #[test]
    fn check_p1_example() {
        assert_eq!(part1(EX).unwrap().as_str(), "5")
    }
    #[test]
    fn check_p2_example() {
        assert_eq!(part2(EX).unwrap().as_str(), "12")
    }
}
