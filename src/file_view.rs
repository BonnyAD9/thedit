use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    ops::Range,
};

use crate::{err::Result, utils::read_to_eof};

struct FileView {
    file: File,
    data: Vec<u8>,
    range: Range<usize>,
}

impl FileView {
    pub fn new(file: File) -> Self {
        Self {
            file,
            data: vec![],
            range: 0..0,
        }
    }

    pub fn try_view(&self, range: Range<usize>) -> Option<&[u8]> {
        assert!(range.start <= range.end);
        if range.start > self.range.start && range.end <= self.range.end {
            Some(self.view_unchecked(range))
        } else {
            None
        }
    }

    pub fn view(&mut self, range: Range<usize>) -> Result<&[u8]> {
        assert!(range.start <= range.end);
        if range.start > self.range.start && range.end <= self.range.end {
            return Ok(self.view_unchecked(range));
        }

        let bneed = to_blocks(range.clone());
        let need_len = from_block(bneed.end - bneed.start);
        self.data.resize(need_len, 0);
        let start = from_block(bneed.start);
        self.file.seek(SeekFrom::Start(start as u64))?;

        let red = read_to_eof(&mut self.file, &mut self.data)?;
        self.data.truncate(red);

        self.range = start..start + red;

        Ok(&self.data[range.start - self.range.end
            ..red.min(range.end - self.range.end)])
    }

    fn view_unchecked(&self, range: Range<usize>) -> &[u8] {
        &self.data[range.start - self.range.start..range.end - self.range.end]
    }
}

const BLOCK_SIZE: usize = 4096;

fn to_block(num: usize) -> usize {
    num / BLOCK_SIZE
}

fn to_blocks(range: Range<usize>) -> Range<usize> {
    to_block(range.start)..to_block(range.end.next_multiple_of(BLOCK_SIZE))
}

fn from_block(num: usize) -> usize {
    num * BLOCK_SIZE
}
