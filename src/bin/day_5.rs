use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("./resc/day_5_original.txt").unwrap();
    let buf = BufReader::new(file);

    let mut cargo_bay: Vec<VecDeque<char>> = Vec::new();
    let mut bay0 = VecDeque::<char>::new();
    bay0.push_back('D');
    bay0.push_back('T');

    cargo_bay.push(bay0);

    let first = cargo_bay.get(0).unwrap().get(0).unwrap();

    println!("first: {} ", *first);
    for line in buf.lines() {
        let str: String = line.unwrap();
        println!("{}", str);
    }
}
