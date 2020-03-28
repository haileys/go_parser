use std::borrow::Borrow;
use std::cmp;
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Loc {
    byte_range: Range<usize>,
}

impl Loc {
    pub fn new(byte_range: Range<usize>) -> Self {
        Loc { byte_range }
    }

    pub fn start(&self) -> usize {
        self.byte_range.start
    }

    pub fn end(&self) -> usize {
        self.byte_range.end
    }

    pub fn range(&self) -> Range<usize> {
        self.byte_range.clone()
    }

    pub fn join(&self, other: impl Borrow<Loc>) -> Loc {
        let other = other.borrow();
        let start = cmp::min(self.byte_range.start, other.byte_range.start);
        let end = cmp::max(self.byte_range.end, other.byte_range.end);

        Loc { byte_range: start..end }
    }
}

// needed for lalrpop:
impl Default for Loc {
    fn default() -> Self {
        Loc { byte_range: 0..0 }
    }
}
