use std::fs::File;
use std::iter::Iterator;
use std::io::Read;


const WINDOW_SIZE: usize = 1 << 13;
const MODULO: u64 = 1 << 12;

pub struct Slicer {
    file: File,
    window_start: usize,
    sum: u64,
    bytes: Vec<Box<[u8; WINDOW_SIZE]>>,
    chunk_start: usize,
    chunk_stop: usize,
    done: bool,
    read_err: bool,
}

impl Slicer {
    pub fn new(file: &str) -> Slicer {
        // Init
        let mut buf: Box<[u8; WINDOW_SIZE]> = Box::new([0; WINDOW_SIZE]);
        let mut vec: Vec <Box<[u8; WINDOW_SIZE]>> = Vec::<Box<[u8; WINDOW_SIZE]>>::with_capacity(10);
        let mut done = false;
        let mut sum: u64 = 0;
        let mut read_err = false;

        // Open file and read first bytes to the buffer to fill the window
        let mut f = File::open(file).expect("");
        let res = match f.read_exact(&mut *buf) {
            Ok(_) => {
                WINDOW_SIZE
            },
            Err(_) => {
                read_err = true;
                0
            },
        };
        if res == 0 {
            done = true;
        }

        // Fill the window initially
        for i in 0..WINDOW_SIZE {
            sum += buf[i] as u64;
        }

        vec.push(buf);

        // Create initialized structure
        Slicer {
            file: f,
            window_start: 0,
            chunk_start: 0,
            chunk_stop: WINDOW_SIZE - 1,
            sum: sum,
            bytes: vec,
            done: done,
            read_err: read_err,
        }
    }
}

impl Iterator for Slicer {
    type Item = (usize, usize);
    
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut remaining = WINDOW_SIZE - (self.chunk_stop % WINDOW_SIZE) - 1;

            if remaining == 0 {
                if !self.done {
                        // Old samples done, fetch new ones
                        let mut buf: Box<[u8; WINDOW_SIZE]> = Box::new([0; WINDOW_SIZE]);
                        let res = match self.file.read_exact(&mut *buf) {
                            Ok(_) => {
                                WINDOW_SIZE
                            },
                            Err(_) => {
                                self.read_err = true;
                                0
                            },
                        };
                        if res == 0 {
                            return None;
                        }
                        self.bytes.push(buf);
                        remaining = WINDOW_SIZE;
                    }
                else {
                    return None;
                }
            }

            for _ in 0..remaining {
                // Calculate new sum
                self.window_start += 1;
                self.chunk_stop += 1;

                let offset = self.chunk_stop / WINDOW_SIZE;
                let pos = self.chunk_stop % WINDOW_SIZE;
                
                self.sum += self.bytes[offset][pos] as u64;

                let offset = self.window_start / WINDOW_SIZE;
                let pos = self.window_start % WINDOW_SIZE;

                self.sum -= self.bytes[offset][pos] as u64;
                
                if self.sum % MODULO == 0 {
                    let start = self.chunk_start;
                    self.chunk_start = self.chunk_stop + 1;
                    return Some((start, self.chunk_stop));
                }
            }
        }
    }
}