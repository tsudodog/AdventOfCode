pub fn hello() -> bool {
    true
}

pub fn compare_schedules(a1: &i32, a2: &i32, b1: &i32, b2: &i32) -> i32 {
    if *a1 <= *b1 && *a2 >= *b2 {
        return 1;
    }

    if *b1 <= *a1 && *b2 >= *a2 {
        return 1;
    }

    return 0;
}

pub fn overlap_at_all(a1: &i32, a2: &i32, b1: &i32, b2: &i32) -> i32 {
    if *a1 <= *b1 && *a2 >= *b1 {
        return 1;
    }

    if *a1 >= *b1 && *a1 <= *b2 {
        return 1;
    }
    return 0;
}
