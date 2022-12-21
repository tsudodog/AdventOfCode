pub mod d4func;
pub mod d5func;
use std::collections::HashSet;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn determine_common_component(s: &String) -> char {
    // split in half
    let half = s.len() / 2;
    let mut chars = HashSet::new();

    let mut x = 0;
    while x < half {
        let ch = match s.chars().nth(x) {
            Some(x) => x,
            None => panic!(),
        };
        chars.insert(ch);
        x = x + 1;
    }

    let mut match_value: Option<char> = None;
    while x < s.len() {
        let ch = match s.chars().nth(x) {
            Some(x) => x,
            None => panic!(),
        };
        if chars.contains(&ch) {
            // we have found our match
            match_value = Some(ch);
            break;
        }
        x = x + 1;
    }

    match match_value {
        Some(x) => x,
        None => panic!(),
    }
}

pub fn union_char(strings: &Vec<String>) -> char {
    let mut sets: Vec<HashSet<char>> = Vec::new();

    for s in strings {
        let mut set = HashSet::new();

        for c in s.to_string().chars() {
            set.insert(c);
        }

        sets.push(set)
    }

    let a_set = sets.get(0).unwrap();
    let b_set = sets.get(1).unwrap();
    let c_set = sets.get(2).unwrap();

    let ab_intersection: HashSet<_> = a_set.intersection(&b_set).copied().collect();

    let abc_intersection: HashSet<_> = ab_intersection.intersection(c_set).copied().collect();

    for x in &abc_intersection {
        println!("{}", x);
    }

    let binding = abc_intersection.iter().nth(0);
    let result = match binding {
        Some(x) => x,
        None => panic!(),
    };

    *result
}

pub fn comp_value(c: char) -> i32 {
    let s = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

    let val = match s.find(c) {
        Some(x) => x,
        None => panic!(),
    } as i32;

    val
}
