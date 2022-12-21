#[cfg(test)]
pub mod day_five_tests {
    use helper_funcs::d5func as d5;

    #[test]
    fn test_hello() {
        let r = d5::hello_from_d5();
        assert_eq!(r, true);
    }
}
