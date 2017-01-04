mod slicer;
mod hasher;

fn main(){
    //let mut f = File::open("testfile").expect("Testfile was not found.");

    let mut sum = 0;
    let s = slicer::Slicer::new("testfile").unwrap();
    for slice in s {
        println!("{:?}", slice.len());
        sum += slice.len();
        println!("{:?}", sum);
    }

}