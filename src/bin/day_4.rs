use helper_funcs::d4func as d4;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
fn main() {
    println!("day 4 code");

    let file = match File::open("./resc/day_4.txt") {
        Ok(it) => it,
        Err(_) => panic!("my message here"),
    };

    let buffered = BufReader::new(file);
    let mut total = 0;
    for line in buffered.lines() {
        let str = match line {
            Ok(x) => x,
            Err(e) => panic!("{}", e.to_string()),
        };

        // let elfs: Vec<&str>= str.split(',').collect();
        let str_list: Vec<&str> = str.split(",").collect();
        let nest_list = str_list
            .iter()
            .map(|&s| {
                s.split("-")
                    .map(|x| FromStr::from_str(x).unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        let first_range = match nest_list.get(0) {
            Some(x) => x,
            None => panic!(),
        };
        let second_range = match nest_list.get(1) {
            Some(x) => x,
            None => panic!(),
        };

        let a1 = first_range.get(0).unwrap();
        let a2 = first_range.get(1).unwrap();
        let b1 = second_range.get(0).unwrap();
        let b2 = second_range.get(1).unwrap();

        let res = d4::overlap_at_all(a1, a2, b1, b2);
        total = total + res;
        println!("a1 {} a2 {} b1 {} b2 {} comp {}", a1, a2, b1, b2, res);
    }

    println!("total: {}", total);
}
