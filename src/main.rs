extern crate flate2;

mod slicer;

use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use flate2::write::ZlibEncoder;
use flate2::Compression;

fn main(){
    //let mut f = File::open("testfile").expect("Testfile was not found.");

    let s = slicer::Slicer::new("testfile");
    for slice in s {
        println!("{:?}", slice);
    }

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
