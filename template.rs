use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Cannot read the file");

   for line in contents.lines() {
   }
}
