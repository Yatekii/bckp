use std::fs::File;
use std::iter::Iterator;
use std::io::Read;
use std::fs::metadata;
use std::collections:VecDeque;


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
        let mut chunk = Vec::<u8>::with_capacity(WINDOW_SIZE);
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
        let res = match f.read(&buf) {
            Ok(n) => { n },
            Err(_) => { read_err = true; 0 },
        };
        if res == 0 {
            done = true;
        }

        // Calculate initial sum
        for i in 0..res {
            sum += buf[i] as u64;
            chunk.push(buf[i]);
            window.push_back(buf[i]);
        }

        // Create initialized structure
        Slicer {
            file: f,
            window: window,
            chunk: chunk,
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
    type Item = (usize, usize);
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut remaining = WINDOW_SIZE - pos;

            if remaining == 0 {
                if !self.done {
                    let res = match self.file.read(&mut self.buffer) {
                        Ok(n) => { n },
                        Err(_) => { self.read_err = true; 0 },
                    };
                    if res == 0 {
                        return None;
                    }
                    self.bytes.push(buf);
                } else {
                    return None;
                }
            }

            for _ in 0..remaining {
                // Calculate new sum
                let mut offset = self.window_start / WINDOW_SIZE;
                let mut pos = self.window_start % WINDOW_SIZE;

                self.sum -= self.bytes[offset][pos] as u64;

                self.window_start += 1;
                self.chunk_stop += 1;

                offset = self.chunk_stop / WINDOW_SIZE;
                pos = self.chunk_stop % WINDOW_SIZE;
                
                self.sum += self.bytes[offset][pos] as u64;
                
                if self.sum % MODULO == 0 {
                    let start = self.chunk_start;
                    self.chunk_start = self.chunk_stop + 1;
                    return Some((start, self.chunk_stop));
                }
            }

            if self.done {
                return Some((self.chunk_start, self.chunk_stop));
            }
        }
    }
}