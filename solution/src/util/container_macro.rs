/// Create a Matrix from a given element
/// ```
/// use solution::mat;
///
/// let excepted = {
///     let mut v = Vec::new();
///     v.push(vec![1, 2]);
///     v.push(vec![3, 4]);
///     v
/// };
/// assert_eq!(excepted, mat![[1, 2], [3, 4]]);
/// ```
#[macro_export]
macro_rules! mat {
    [ $( [ $( $d:expr ),* ] ),* ] => {
        vec![
            $(
                vec![$($d),*],
            )*
        ]
    }
}

/// Create a Matrix from a given element
/// Then each element will be converted to the type of inference
/// ```
/// use solution::mat_into;
///
/// fn f(m : Vec<Vec<String>>) -> Vec<Vec<String>> { m }
///
/// let excepted = {
///     let mut v: Vec<Vec<String>> = Vec::new();
///     v.push(vec!["1".into(), "2".into()]);
///     v.push(vec!["3".into(), "4".into()]);
///     v
/// };
///
/// // &str into String
/// assert_eq!(excepted, f(mat_into![["1", "2"], ["3", "4"]]));
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

#[macro_export]
macro_rules! vec_char {
    ( $( $ele:expr ) , *) => {
       vec![$($ele.chars().nth(0).unwrap()), *]
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
