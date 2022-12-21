#[cfg(test)]
mod tests_d4 {
    use helper_funcs::d4func as d4;

    #[test]
    fn test_add() {
        let r = d4::hello();
        assert_eq!(r, true);
    }
    #[test]
    fn comp_1() {
        let r = d4::compare_schedules(&6, &6, &4, &6);
        assert_eq!(r, 1);
    }
    #[test]
    fn comp_2() {
        let r = d4::compare_schedules(&2, &8, &3, &7);
        assert_eq!(r, 1);
    }

    #[test]
    fn comp_3() {
        let r = d4::compare_schedules(&2, &4, &6, &8);
        assert_eq!(r, 0);
    }
    #[test]
    fn comp_4() {
        let r = d4::compare_schedules(&2, &62, &62, &98);
        assert_eq!(r, 0)
    }

    #[test]
    fn overlap_1() {
        let r = d4::overlap_at_all(&5, &7, &7, &9);
        assert_eq!(r, 1);
    }
    #[test]
    fn overlap_2() {
        let r = d4::overlap_at_all(&2, &8, &3, &7);
        assert_eq!(r, 1);
    }
    #[test]
    fn overlap_3() {
        let r = d4::overlap_at_all(&6, &6, &4, &6);
        assert_eq!(r, 1);
    }

    #[test]
    fn overlap_4() {
        let r = d4::overlap_at_all(&2, &6, &4, &8);
        assert_eq!(r, 1);
    }

    #[test]
    fn overlap_5() {
        let r = d4::overlap_at_all(&1, &3, &4, &8);
        assert_eq!(r, 0);
    }
}
