extern crate flate2;

mod slicer;

use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use std::collections::VecDeque;
use flate2::write::ZlibEncoder;
use flate2::Compression;

fn main(){
    let mut f = File::open("testfile").expect("Testfile was not found.");

    let window_size = 1 << 13;
    let modulo = 1 << 12;

    let mut buffer: [u8; 1 << 13] = [0; 1 << 13];
    let mut res;
    let mut n: u64 = 0;

    let mut total: u64 = 0;
    let mut sum: u64 = 0;

    let mut ckto = 0;

    let mut chunks = Vec::<u64>::new();
    let mut window = VecDeque::<u8>::new();

    res = f.read(&mut buffer);
    match res {
        Ok(v) => n = v as u64,
        Err(_) => println!("File could not be read.")
    }
    // Read bytes until file reaches EOF
    while n != 0 {
        if total + n < window_size {
            // Buffer is nowhere full so just sum all bytes and continue reading more bytes
            total += n;
            for i in 0..(n as usize) {
                sum += buffer[i] as u64;
                window.push_back(buffer[i] as u8);
            }
        } else {
            // Buffer is nearly full for a first window so just read a few bytes and sum em
            for i in 0..((window_size - total) as usize) {
                sum += buffer[i] as u64;
                window.push_back(buffer[i] as u8);
            }

            total = window_size;

            // Initial window filled, so check first sum
            if 0 == sum % modulo {
                let last = *chunks.last().unwrap_or(&0);
                println!("{:?}", (last, total));
                //write_compressed_chunk(total as u64, last as u64, "testfile".to_string(), format!("file_{}", total));

                chunks.push(total);
            }

            // Iterate over all remaining bytes with the moving window and check their sums
            while n != 0 {
                res = f.read(&mut buffer);
                match res {
                    Ok(v) => n = v as u64,
                    Err(_) => println!("File could not be read.")
                }
                for i in 0..(n as usize) {
                    let pop = window.pop_front().unwrap();
                    sum += buffer[i] as u64;
                    sum -= pop as u64;
                    window.push_back(buffer[i]);
                    if 0 == sum % modulo {
                        let last = *chunks.last().unwrap_or(&0);
                        ckto += total + (i as u64) - last;
                        println!("{:?}", (last, total + (i as u64)));
                        //write_compressed_chunk(total + (i as u64), last as u64, "testfile".to_string(), format!("file_{}", total + (i as u64)));

                        chunks.push(total + (i as u64));
                    }
                }
                total += n;
            }
        }
        res = f.read(&mut buffer);
        match res {
            Ok(v) => n = v as u64,
            Err(_) => println!("File could not be read.")
        }
    }

    let last = *chunks.last().unwrap_or(&0);
    ckto += total - last;

    println!("{:?}", (last, total));
    //write_compressed_chunk(total as u64, last as u64, "testfile".to_string(), format!("file_{}", total));

    chunks.push(total);
    //println!("{:?}", chunks);
    //println!("{}", ckto);
    //write_compressed_chunk(total, 0, "testfile".to_string(), "KEK".to_string());

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
