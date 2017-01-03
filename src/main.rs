mod slicer;
mod hasher;

use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;

fn main(){
    //let mut f = File::open("testfile").expect("Testfile was not found.");

    let s = slicer::Slicer::new("testfile");
    for slice in s {
        println!("{:?}", slice);
    }

}