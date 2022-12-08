use std::fs;

#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
enum Overlap {
    None, Full, Partial
}

fn main() {
    let file_str = fs::read_to_string("day-4.input").expect("Failed to read file");

    let overlaps : Vec<Overlap> = file_str.trim().split("\n").map(|line| {
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

        if (e1_start == e2_start) || (e1_end == e2_end){
            // If they share an endpoint, one surely contains or equals the other
            Overlap::Full
        } else if (e1_start < e2_start) == (e1_end > e2_end)  {
            // e1_start's direction from e2_start is the opposite from the respective direction of
            // e1_end and e2_end. This means one fully includes the other.
            Overlap::Full
        } else if (e1_start < e2_end) || (e2_start < e1_end) {
            // One is strictly before the other. No overlap.
            Overlap::None
        } else {
            // The only remaining possiblity is a partial overlap
            Overlap::Partial
        }
    }).collect();

    let num_fulls = overlaps.iter().filter_map(|&overlap| match overlap{Overlap::Full => Some(()), _ => None}).count();
    let num_partials = overlaps.iter().filter_map(|&overlap| match overlap{Overlap::Partial => Some(()), _ => None}).count();

    println!("Overlaps: full={} partial={}", num_fulls, num_partials)
}
