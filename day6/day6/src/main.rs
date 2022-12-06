use std::fs;

/*

bvwbjplbgvbhsrlpgdmjqwftvncz

bvwb      -  b/v b/w b/b, v/w v/b, w/b
 vwbj     -          v/j,     w/j, b/j
  wbjp    -          w/p,     b/p, j/p
   bjpl   - 
    jplb  - 


bb__ offset += 1
b_b_ offset += 1
b__b offset += 1
_b_b offset += 2
_bb_ offset += 3
__bb offset += 4


AABC 

part1 - 1282
part2 - 3513

*/

fn dcode(marker_length: usize) {
    let contents = fs::read_to_string("input").expect("Cannot read the file");

    let mut offset = 0;
    'main: loop {
        let current_marker = &contents[offset..offset + marker_length];
        'marker: for (i, c1) in current_marker.chars().enumerate() {
            for c2 in current_marker.chars().skip(i + 1) {
                if c1 == c2 {
                    offset += i + 1;
                    break 'marker;
                }
            }
            if i == marker_length - 1 {
                break 'main;
            }
        }
    }
    println!("dcode: {}", offset + marker_length);
}

fn main() {
    dcode( 4);
    dcode(14);
}
