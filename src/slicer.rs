use std::fs::File;
use std::iter::Iterator;
use std::io::Read;
use std::collections::VecDeque;

// Yatekii: Okay, so I'd suggest: when you detect the end of a chunk,
// use Vec::split_off to get a new vector holding the tail that isn't
// part of that chunk, and remove that tail from your buffer; then use
// std::mem::swap to exchange the new tail vector with self.chunk,
// and then return the chunk.

pub const WINDOW_SIZE: usize = 1 << 13;
pub const MODULO: u64 = 1 << 12;

pub struct Slicer {
    file: File,
    sum: u64,
    window: VecDeque<u8>,
    buffer: [u8; WINDOW_SIZE],
    len: usize,
    pos: usize,
    read_err: bool,
}

impl Slicer {
    pub fn new(file: &str) -> Option<Slicer> {
        // Init
        let window = VecDeque::<u8>::with_capacity(WINDOW_SIZE);
        let buf = [0 as u8; WINDOW_SIZE];

        // Open file and read first bytes to the buffer to fill the window
        match File::open(file) {
            Ok(v) => { 
                // Create initialized structure
                Some(Slicer {
                    file: v,
                    window: window,
                    buffer: buf,
                    len: 0,
                    pos: 0,
                    sum: 0,
                    read_err: false,
                })
             },
            Err(_) => { None }
        }
    }
}

impl Iterator for Slicer {
    type Item = Box<Vec<u8>>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::<u8>::with_capacity(WINDOW_SIZE);
        loop {
            let remaining = self.len - self.pos;

            if remaining == 0 {
                let res = match self.file.read(&mut self.buffer) {
                    Ok(n) => { self.pos = 0; self.len = n; n },
                    Err(_) => { self.read_err = true; 0 },
                };
                if res == 0 {
                    if chunk.len() != 0 && !self.read_err {
                        return Some(Box::new(chunk));
                    }
                    return None;
                }
            }

            while self.pos < self.len {
                // Calculate new sum
                self.sum += self.buffer[self.pos] as u64;
                self.window.push_back(self.buffer[self.pos]);
                
                chunk.push(self.buffer[self.pos]);

                self.pos += 1;

                if self.window.len() == WINDOW_SIZE + 1 {
                    self.sum -= self.window.pop_front().unwrap() as u64;
                    if self.sum % MODULO == 0 {
                        return Some(Box::new(chunk));
                    }
                }
            }
        }
    }
}