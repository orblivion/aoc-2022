use std::fs;

fn main() {
    let file_str = fs::read_to_string("day-4.input").expect("Failed to read file");

    let overlaps = file_str.trim().split("\n").filter_map(|line| {
        let (elf1, elf2) = line.split_at(line.find(",").unwrap());

        let elf2 = &elf2[1..];

        let (e1_start, e1_end) = elf1.split_at(elf1.find("-").unwrap());
        let (e2_start, e2_end) = elf2.split_at(elf2.find("-").unwrap());

        let e1_end = &e1_end[1..];
        let e2_end = &e2_end[1..];

        let e1_start = e1_start.to_string().parse::<i32>().unwrap();
        let e2_start = e2_start.to_string().parse::<i32>().unwrap();
        let e1_end = e1_end.to_string().parse::<i32>().unwrap();
        let e2_end = e2_end.to_string().parse::<i32>().unwrap();

        // If they share an endpoint, one surely contains or equals the other
        if e1_start == e2_start {
            Some(())
        } else if e1_end == e2_end {
            Some(())
        } else if (e1_start < e2_start) == (e1_end > e2_end)  {
            Some(())
        } else {
            // Since start and end times aren't respectively equal, we can ignore <= or => possibilities
            // So let's check that the order of the starts is the opposite of the order of the ends
            None
        }
    })
    .count();

    println!("Overlaps: {}", overlaps)
}
