use std::fs::File;
use std::io::{BufRead, BufReader};

struct MatchResult {
    score: i32,
}

fn compute_match(p1: String, p2: String) -> MatchResult {
    if p1.eq("A") {
        if p2.eq("X") {
            println!("A X hit! Ret 0, 4");
            return MatchResult {
        score : 3 // was 4
      };
        }

        if p2.eq("Y") {
            return MatchResult{
        score: 4 // 8
      };
        }
        if p2.eq("Z") {
            return MatchResult{
        score : 8 //3
      };
        }
    }

    if p1.eq("B") {
        if p2.eq("X") {
            return MatchResult{
        score:  1//1
      };
        }
        if p2.eq("Y") {
            return MatchResult{
        score: 5//5
      };
        }
        if p2.eq("Z") {
            return MatchResult{
        score :  9//9
      };
        }
    }

    if p1.eq("C") {
        if p2.eq("X") {
            return MatchResult{
        score : 2//7
      };
        }
        if p2.eq("Y") {
            return MatchResult{
        score: 6//2
      };
        }
        if p2.eq("Z") {
            return MatchResult{
        score: 7//6
      };
        }
    }

    MatchResult { score: 0 }
}

fn main() {
    println!("hello day 2");
    let file = match File::open("/home/tsudo/rust_projects/test_rust/resc/day_2.txt") {
        Ok(it) => it,
        Err(_) => panic!(),
    };

    let bufffered = BufReader::new(file);
    let mut total: i32 = 0;
    for line in bufffered.lines() {
        let str = match line {
            Ok(k) => k,
            Err(e) => panic!("{}", e.to_string()),
        };

        let a = str.split(" ");
        let vec: Vec<&str> = a.collect();

        println!("{}", str);

        let zero = match vec.get(0) {
            Some(x) => x,
            None => panic!(),
        };
        let one = match vec.get(1) {
            Some(x) => x,
            None => panic!(),
        };
        let match_result = compute_match(zero.to_string(), one.to_string());
        total = total + match_result.score;
        println!("zero: {} one: {}", zero, one);
    }
    println!("{}", total);
}
