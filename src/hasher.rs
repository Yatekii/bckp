/*extern crate ring;
extern crate flate2;

use slicer;
use self::ring::digest;
use self::ring::digest::Digest;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use self::flate2::write::ZlibEncoder;
use self::flate2::Compression;  

pub struct Hasher {

}

pub fn hash(data: &slicer::Slicer, start: usize, stop: usize) -> Digest {
    let size = start - stop;
    let mut vec = Vec::<u8>::with_capacity(size);
    let start_offset = start / slicer::WINDOW_SIZE;
    let start_pos = start % slicer::WINDOW_SIZE;
    let stop_offset = stop / slicer::WINDOW_SIZE;
    let stop_pos = stop / slicer::WINDOW_SIZE;

    for x in start_pos..slicer::WINDOW_SIZE {
        vec.push(data.bytes[start_offset][x]);
    }
    for y in (start_offset - 1)..stop_offset {
        for x in 0..slicer::WINDOW_SIZE {
            vec.push(data.bytes[y][x]);
        }
    }
    for x in start_pos..slicer::WINDOW_SIZE {
        vec.push(data.bytes[stop_offset][x]);
    }
    return digest::digest(&digest::SHA256, vec.as_mut_slice());
}

fn write_compressed_chunk(total: u64, last: u64, src_file: String, trg_file: String) {
    let mut buf: Vec<u8> = Vec::<u8>::new();
    buf.resize((total - last) as usize, 0);
    let mut src = File::open(src_file).expect("Testfile could not be operated.");
    let _ = src.seek(SeekFrom::Start(last as u64));
    let _ = src.read_exact(buf.as_mut_slice());

    let chunk_file = File::create(format!("comp/{}", &trg_file)).expect("Chunkfile could not be operated.");
    let mut uncompressed_file = File::create(format!("un/{}", &trg_file)).expect("Chunkfile could not be operated.");
    let mut encoder = ZlibEncoder::new(chunk_file, Compression::Default);
    let _ = encoder.write(buf.as_slice());
    let _ = uncompressed_file.write(buf.as_slice());
}
*/