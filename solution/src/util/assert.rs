#[macro_export]
macro_rules! assert_ignored_order {
    ($a:expr, $b:expr) => {
        let mut a = $a;
        let mut b = $b;
        a.sort();
        b.sort();
        assert_eq!(a, b);
    };
}
