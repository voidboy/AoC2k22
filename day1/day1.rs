use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Cannot read the file");

   let mut current: usize = 0;
   let mut top3: [usize; 3] = [0; 3];
   for line in contents.lines() {
       if line.is_empty() {
           if current > top3[0] {
               top3[0] = current;
           }
           top3.sort();
           current = 0;
       } else {
           current += line.parse::<usize>()
               .expect("Not a number");
       }
   }
   let mut top3_elf_calories: usize = 0;
   top3.map(|n| { top3_elf_calories += n });
   println!("The winner is {}", top3_elf_calories);
}
