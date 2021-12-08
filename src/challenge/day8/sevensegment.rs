use anyhow::Result;

const SEGMENTS_LEN: usize = 7;

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
    pub fn from_wire_code(code: &str) -> Result<SevenSegment> {
        let mut disp = SevenSegment::default();
        for idx in code.chars().map(wire_name_to_pos) {
            let idx = idx?;
            disp.0[idx] = true
        }
        Ok(disp)
    }
    fn lit_segments(self) -> usize {
        self.0.iter().filter(|s| **s).count()
    }
}

pub fn guess_digit(s: SevenSegment) -> Option<i64> {
    match s.lit_segments() {
        2 => Some(1),
        4 => Some(4),
        3 => Some(7),
        7 => Some(8),
        _ => None,
    }
}
