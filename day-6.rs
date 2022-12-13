use std::fs;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {

    let file_str = fs::read_to_string("day-6.input").expect("Failed to read file");

    let mut marker : VecDeque<char> = VecDeque::new();
    let mut index : usize = 0;

    for (i, ch) in file_str.trim().chars().enumerate() {
        marker.push_front(ch);
        if marker.len() > 4 {
            marker.pop_back();
        }

        let marker_uniq : HashSet<char> = marker.iter().map(|&x| x).collect();
        if marker_uniq.len() == 4 {
            index = i + 1;
            break;
        }
    }

    println!("{:?}", index);
}
