use std::fs::File;
use std::iter::Iterator;
use std::io::Read;
use std::fs::metadata;
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
    chunk: Vec<u8>,
    buffer: [u8; WINDOW_SIZE],
    pos: usize,
    done: bool,
    read_err: bool,
    size: usize,
    read_n: usize,
}

impl Slicer {
    pub fn new(file: &str) -> Slicer {
        // Init
        let mut window = VecDeque::<u8>::with_capacity(WINDOW_SIZE);
        let mut done = false;
        let mut sum: u64 = 0;
        let mut read_err = false;
        let mut buf = [0 as u8; WINDOW_SIZE];
        let meta = metadata(file);
        let size = match meta {
            Ok(v) => { v.len() as usize },
            Err(_) => { read_err = true; 0 }
        };

        // Open file and read first bytes to the buffer to fill the window
        let mut f = File::open(file).expect("");

        // Create initialized structure
        Slicer {
            file: f,
            window: window,
            buffer: buf,
            pos: res,
            sum: sum,
            done: done,
            read_err: read_err,
            size: size,
            read_n: WINDOW_SIZE,
        }
    }
}

impl Iterator for Slicer {
    type Item = Vec<u8>;
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut remaining = self.buffer.len() - self.pos;

            if !self.done {
                if remaining == 0 {
                    self.chunk = Vec::<u8>::with_capacity(WINDOW_SIZE);
                    let res = match self.file.read(&mut self.buffer) {
                        Ok(n) => { n },
                        Err(_) => { self.read_err = true; 0 },
                    };
                    if res == 0 {
                        return None;
                    }
                }
            } else {
                return None;
            }

            while self.pos < self.buffer.len() {
                // Calculate new sum
                self.sum += self.buffer[self.pos] as u64;
                self.chunk.push(self.buffer[self.pos]);
                self.window.push_back(self.buffer[self.pos]);
                self.sum -= self.window.pop_front().unwrap() as u64;
                
                if self.sum % MODULO == 0 {
                    return Some(self.chunk);
                }
            }

            if self.done {
                return Some(self.chunk);
            }
            self.pos += 1;
        }
    }
}