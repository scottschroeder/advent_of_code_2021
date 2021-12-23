use anyhow::Result;

pub const SEGMENTS_LEN: usize = 7;

/*
     aaaa
    b    c
    b    c
     dddd
    e    f
    e    f
     gggg
*/
#[derive(Debug, Clone, Copy, Default)]
pub struct SevenSegment([bool; SEGMENTS_LEN]);

fn wire_name_to_pos(c: char) -> Result<usize> {
    Ok(match c {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => anyhow::bail!("wire {:?} does not exist", c),
    })
}

impl SevenSegment {
    pub fn from_last_digit(d: u8) -> SevenSegment {
        SevenSegment(match d % 10 {
            0 => [true, true, true, false, true, true, true],
            1 => [false, false, true, false, false, true, false],
            2 => [true, false, true, true, true, false, true],
            3 => [true, false, true, true, false, true, true],
            4 => [false, true, true, true, false, true, false],
            5 => [true, true, false, true, false, true, true],
            6 => [true, true, false, true, true, true, true],
            7 => [true, false, true, false, false, true, false],
            8 => [true, true, true, true, true, true, true],
            9 => [true, true, true, true, false, true, true],
            _ => unreachable!("invalid digit"),
        })
    }
    pub fn from_wire_code(code: &str) -> Result<SevenSegment> {
        let mut disp = SevenSegment::default();
        for idx in code.chars().map(wire_name_to_pos) {
            let idx = idx?;
            disp.0[idx] = true
        }
        Ok(disp)
    }
    fn lit_segments(&self) -> impl Iterator<Item = usize> + '_ {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(idx, s)| if *s { Some(idx) } else { None })
    }
    fn total_lit_segments(self) -> usize {
        self.lit_segments().count()
    }
}

pub fn guess_digit(s: SevenSegment) -> Option<i64> {
    match s.total_lit_segments() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}
