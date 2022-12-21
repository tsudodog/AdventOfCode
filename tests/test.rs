#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        let result = helper_funcs::add(2, 4);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_determine_common_component_1() {
        let val = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        let result = helper_funcs::determine_common_component(&val);
        assert_eq!(result, 'p');
    }
    #[test]
    fn test_determine_common_component_2() {
        let val = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string();
        let result = helper_funcs::determine_common_component(&val);
        assert_eq!(result, 'L');
    }

    #[test]
    fn test_comp_value() {
        let result = helper_funcs::comp_value('a');
        assert_eq!(result, 1);
    }

    #[test]
    fn test_comp_value2() {
        let result = helper_funcs::comp_value('P');
        assert_eq!(result, 42);
    }

    #[test]
    fn test_union() {
        let mut a = Vec::new();
        a.push("abcdef".to_string());
        a.push("zxyalk".to_string());
        a.push("qrsatv".to_string());
        let result = helper_funcs::union_char(&a);
        assert_eq!(result, 'a');
    }

    #[test]
    fn test_union_1() {
        let mut a = Vec::new();
        a.push("vJrwpWtwJgWrhcsFMMfFFhFp".to_string());
        a.push("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string());
        a.push("PmmdzqPrVvPwwTWBwg".to_string());
        let result = helper_funcs::union_char(&a);
        assert_eq!(result, 'r');
    }
}
