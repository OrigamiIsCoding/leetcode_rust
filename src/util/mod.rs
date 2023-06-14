pub mod i32_singly_linked_list;

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

#[macro_export]
macro_rules! mat_into {
    [ $( [ $( $ele:expr ),* ] ),* ] => {
        vec![
            $(
                vec![$($ele.into()),*],
            )*
        ]
    }
}

#[macro_export]
macro_rules! vec_into {
    ( $( $ele:expr ) , *) => {
       vec![$($ele.into()), *]
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mat() {
        let matrix = mat![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        assert_eq!(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9],], matrix)
    }

    #[test]
    fn test_vec_into() {
        let svec: Vec<String> = vec_into!["123"];
        assert_eq!(vec!["123".to_string()], svec);
    }
}
