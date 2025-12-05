use std::fs;

fn find_duplicate_values(min: &str, max: &str, duplicate_string: &mut Vec<isize>) {
    let parsed_min: isize = min.trim().parse().unwrap();
    let parsed_max: isize = max.trim().parse().unwrap();
    for i in parsed_min..parsed_max + 1 {
        let i_string = i.to_string();
        let mid = i_string.len() / 2;
        let (left, right) = i_string.split_at(mid);
        if left == right {
            let val: isize = i_string.trim().parse().unwrap();
            duplicate_string.push(val);
        }
    }
}

fn main() {
    println!("Advent of Code - Day 2");

    let args: Vec<String> = std::env::args().collect();
    let file_name: &String = &args[1];

    let file_contents = fs::read_to_string(file_name).unwrap();
    let sub_strings: Vec<&str> = file_contents.split(",").map(|item| item).collect();
    let mut duplicate_string: Vec<isize> = Vec::new();
    for item in sub_strings {
        let a: Vec<_> = item.split("-").collect();
        println!("{:?}", a);
        find_duplicate_values(a[0], a[1], &mut duplicate_string);
    }
    let sum: isize = duplicate_string.iter().sum();
    println!("Sum of all duplicated : {:?}", sum)
}
