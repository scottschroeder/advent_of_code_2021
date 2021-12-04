use aoc::grid::{fixed_grid::FixedGrid, point::Point};
use std::fmt;

const BINGO_DIMM: usize = 5;
const BINGO_WIDTH: usize = BINGO_DIMM;
const BINGO_HEIGHT: usize = BINGO_DIMM;
pub const BINGO_LEN: usize = BINGO_WIDTH * BINGO_HEIGHT;
const MAX_NUMBER: usize = 100;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Mark(bool);

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 {
            write!(f, "X")
        } else {
            write!(f, ".")
        }
    }
}

#[derive(Debug)]
struct BingoCard {
    marks: FixedGrid<Mark>,
    won: bool,
}

impl BingoCard {
    fn check_winner_at(&self, idx: usize) -> bool {
        if self.won {
            return false;
        }
        let p = self.marks.idx_to_point(idx);
        if self.check_range(self.row(p.y as usize)) {
            return true;
        }
        if self.check_range(self.col(p.x as usize)) {
            return true;
        }
        // if p.x == p.y && self.check_range(self.diagonal_down()) {
        //     return true;
        // }
        // if p.x == BINGO_HEIGHT as i64 - p.y && self.check_range(self.diagonal_up()) {
        //     return true;
        // }

        false
    }

    fn mark_if_winner(&mut self, idx: usize) -> bool {
        if self.check_winner_at(idx) {
            self.won = true;
            true
        } else {
            false
        }
    }

    fn col(&self, idx: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..BINGO_HEIGHT).map(move |y| (idx, y))
    }
    fn row(&self, idx: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..BINGO_WIDTH).map(move |x| (x, idx))
    }

    fn check_range(&self, points: impl Iterator<Item = (usize, usize)>) -> bool {
        points
            .map(|(x, y)| Point::new(x as i64, y as i64))
            .all(|p| self.marks[p].0)
    }
}

impl Default for BingoCard {
    fn default() -> Self {
        BingoCard {
            marks: FixedGrid::from_dimm(BINGO_HEIGHT, BINGO_WIDTH),
            won: false,
        }
    }
}

#[derive(Debug, Default)]
struct BingoNumberStats {
    seen: bool,
    cards: Vec<(usize, usize)>,
}

pub struct MultiBingo {
    mapping: Vec<BingoNumberStats>,
    cards: Vec<BingoCard>,
}

impl fmt::Debug for MultiBingo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // self.mapping.fmt(f)?;
        for (idx, c) in self.cards.iter().enumerate() {
            write!(f, "CARD #{}\n{}\n", idx, c.marks)?
        }
        Ok(())
    }
}

impl Default for MultiBingo {
    fn default() -> Self {
        let mut mapping = Vec::with_capacity(MAX_NUMBER);
        mapping.resize_with(MAX_NUMBER, BingoNumberStats::default);

        MultiBingo {
            mapping,
            cards: Vec::new(),
        }
    }
}

struct BingoEntry {
    seen: bool,
    card_idx: usize,
    number: u8,
}

impl MultiBingo {
    pub fn add_card(&mut self, data: &[u8]) {
        assert_eq!(
            data.len(),
            BINGO_WIDTH * BINGO_HEIGHT,
            "card was the wrong number of numbers"
        );
        let card_idx = self.cards.len();
        self.cards.push(BingoCard::default());
        for (card_pos, x) in data.iter().enumerate() {
            let idx = *x as usize;
            self.mapping[idx].cards.push((card_idx, card_pos));
        }
    }

    fn flat_iter_bingo(&self) -> impl Iterator<Item = BingoEntry> + '_ {
        self.mapping
            .iter()
            .enumerate()
            .flat_map(|(num_idx, stats)| {
                stats.cards.iter().map(move |(idx, _)| BingoEntry {
                    seen: stats.seen,
                    card_idx: *idx,
                    number: num_idx as u8,
                })
            })
    }

    pub fn unmarked_at_card(&self, card_idx: usize) -> impl Iterator<Item = u8> + '_ {
        self.flat_iter_bingo()
            .filter(move |e| !e.seen && e.card_idx == card_idx)
            .map(|e| e.number)
    }

    pub fn call_number(&mut self, called: u8) -> Option<usize> {
        let idx = called as usize;
        let stats = &mut self.mapping[idx];
        stats.seen = true;
        let mut win = None;
        for (card_idx, card_pos) in &stats.cards {
            let card = &mut self.cards[*card_idx];
            card.marks.inner[*card_pos] = Mark(true);
            if card.mark_if_winner(*card_pos) {
                win = Some(*card_idx);
            }
        }
        win
    }
    pub fn get_winner_details(&self, card_idx: usize) -> u64 {
        log::debug!("winner: \n{}", self.cards[card_idx].marks);
        self.unmarked_at_card(card_idx)
            .map(|x| x as u64)
            .sum::<u64>()
    }

    pub fn iter<I: Iterator<Item = u8>>(&mut self, iter: I) -> BingoWinners<I> {
        BingoWinners { iter, bingo: self }
    }
}

pub struct BingoWinners<'a, I> {
    iter: I,
    bingo: &'a mut MultiBingo,
}

impl<'a, I: Iterator<Item = u8>> Iterator for BingoWinners<'a, I> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        for call in self.iter.by_ref() {
            if let Some(idx) = self.bingo.call_number(call) {
                let total = self.bingo.get_winner_details(idx);
                let win = total * call as u64;
                log::debug!("winner: #{}, {} x {} = {}", idx, total, call, win);
                return Some(win);
            }
        }
        None
    }
}
