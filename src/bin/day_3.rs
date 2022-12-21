use helper_funcs;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Day 3 code");
    let file = match File::open("/home/tsudo/rust_projects/test_rust/resc/day_3.txt") {
        Ok(it) => it,
        Err(_err) => panic!(),
    };

    let buffered = BufReader::new(file);

    let mut total: i32 = 0;
    let mut total2: i32 = 0;
    let mut v = Vec::new();
    for line in buffered.lines() {
        let str = match line {
            Ok(x) => x,
            Err(e) => panic!("{}", e.to_string()),
        };

        let ch = helper_funcs::determine_common_component(&str);
        let score = helper_funcs::comp_value(ch);
        if v.len() < 3 {
            println!("pushing {}", str);
            v.push(str);
        } else {
            let res_char = helper_funcs::union_char(&v);
            println!("group char: {}", res_char);
            let new_score = helper_funcs::comp_value(res_char);
            v.clear();
            total2 = total2 + new_score;
            println!("pushing {}", str);
            v.push(str);
        }
        total = total + score;
        // println!("ch: {} score: {}",ch, score);
    }

    println!("vlen{}", v.len());
    if v.len() == 3 {
        let res_char = helper_funcs::union_char(&v);
        println!("group char: {}", res_char);
        let new_score = helper_funcs::comp_value(res_char);
        println!("new score {}", new_score);
    }

    println!("The final total is {}", total);
    println!("The final second total is {}", total2);
}
