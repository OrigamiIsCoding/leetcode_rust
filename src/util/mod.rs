#[macro_export]
macro_rules! mat{
    [ $( [ $( $d:expr ),* ] ),* ] => {
        vec![
            $(
                vec![$($d),*],
            )*
        ]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mat() {
        let matrix = mat![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        assert_eq!(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],], matrix)
    }
}
