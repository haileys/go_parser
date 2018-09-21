use std::cmp;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Debug)]
pub struct Loc {
    path: Rc<PathBuf>,
    byte_range: Range<usize>,
}

impl Loc {
    pub fn new(path: Rc<PathBuf>, byte_range: Range<usize>) -> Self {
        Loc { path, byte_range }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn range(&self) -> Range<usize> {
        self.byte_range.clone()
    }

    pub fn join(&self, other: &Loc) -> Loc {
        if self.path != other.path {
            panic!("can't join locs from disparate files");
        }

        let start = cmp::min(self.byte_range.start, other.byte_range.start);
        let end = cmp::max(self.byte_range.end, other.byte_range.end);

        Loc {
            path: Rc::clone(&self.path),
            byte_range: start..end,
        }
    }
}
