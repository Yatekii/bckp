use std::fs::File;
pub struct Slicer {
    file: File,
}

impl Slicer {
    pub fn new(file: &str) -> Slicer {
        let f = File::open(file).expect("");
        Slicer {
            file: f,
        }
    }
}