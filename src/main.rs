mod slicer;
mod hasher;

fn main(){
    //let mut f = File::open("testfile").expect("Testfile was not found.");

    let s = slicer::Slicer::new("testfile").unwrap();
    for slice in s {
        // println!("{:?}", slice.len());
    }

}