use std::ops::Range;

#[derive(Debug)]
pub struct Loc {
    byte_range: Range<usize>,
}

impl Loc {
    pub fn new(byte_range: Range<usize>) -> Self {
        Loc { byte_range }
    }

    pub fn range(&self) -> Range<usize> {
        self.byte_range.clone()
    }
}
