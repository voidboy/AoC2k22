use std::fs;
use std::iter::zip;

/*
    Part. I : 6225 too High
*/

fn split_packet(packet: &str) -> Vec<&str> {
    let mut values = vec![];
    let mut square_bracket_level = 0;
    let mut last = 0;
    for (i, c) in packet.as_bytes().iter().enumerate() {
        match c {
            b'[' => {
                square_bracket_level += 1;
                if square_bracket_level == 1 {
                    last = i + 1;
                }
            }
            b']' => {
                square_bracket_level -= 1;
                if square_bracket_level == 0 {
                    // skip empty strings 
                    if last != i { 
                        values.push(&packet[last..i]);
                    }
                }
            }
            b',' => {
                if square_bracket_level == 1 {
                    // skip empty strings 
                    if last != i {
                        values.push(&packet[last..i]);
                    }
                    last = i + 1;
                }
            }
            b'0'..=b'9' => continue,
            _ => panic!("Invalid packet"),
        }
    }
    values
}

fn compare_packets(
    left_packet_values: &Vec<&str>,
    right_packet_values: &Vec<&str>,
    i: usize,
) -> usize {
    //println!("--- {:?} - {:?} ---", left_packet_values, right_packet_values);
    for (lvalue, rvalue) in zip(left_packet_values, right_packet_values) {
        //println!("{:?} - {:?}", lvalue, rvalue);
        match (lvalue.len(), rvalue.len()) {
            // CASE 1 : If both values are integers
            (1, 1) => {
                // left is lower than right, inputs in right order
                if lvalue.as_bytes()[0] < rvalue.as_bytes()[0] {
                    //println!("OK");
                    return i + 1;
                }
                // left is higher than right, inputs NOT in right order
                else if lvalue.as_bytes()[0] > rvalue.as_bytes()[0] {
                    //println!("KO");
                    return 0;
                }
                // left is equal to right, continue checking
                else {
                    continue;
                }
            }
            // CASE 2 : If both values are lists
            (2.., 2..) => {
                if lvalue == rvalue { 
                    continue ;
                } else {
                    return compare_packets(&split_packet(lvalue), &split_packet(rvalue), i);
                }
            }
            // CASE 3 : If exactly one value is an integer
            (_, 1) => {
                if lvalue.as_bytes()[1] < rvalue.as_bytes()[0] {
                    return i + 1;
                // left > right or right run out of values
                } else {
                    return 0;
                }
            }
            (1, _) => {
                if lvalue.as_bytes()[0] > rvalue.as_bytes()[1] {
                    return 0;
                // left < right or left run out of values
                } else {
                    return i + 1;
                }
            }
            (_, _) => panic!("Unsupported comparaison"),
        }
    }
    // if we reach this, that means we consumed all the input, we must identify if
    // it is left or right ran out of values, otherwise, we must check next values
    if left_packet_values.len() < right_packet_values.len() {
        return i + 1;
    } else {
        return 0;
    }
}

fn main() {
    let contents = fs::read_to_string("my_input").expect("Cannot read the file");

    let part1: usize = contents
        .split("\n\n")
        .enumerate()
        .map(|(i, lines)| {
            let mut pairs = lines.split('\n');
            let left_pair = pairs.next().unwrap();
            let right_pair = pairs.next().unwrap();
            let left_packet_values = split_packet(left_pair);
            let right_packet_values = split_packet(right_pair);
            compare_packets(&left_packet_values, &right_packet_values, i)
        })
        .sum();
    println!("Part. I: {part1}");
}
