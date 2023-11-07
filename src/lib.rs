/// Creates a vector of vector with vecc!, instead of repeating vec! multiple times.
///
/// Examaple: `let v = vecc![[1, 2], [3, 4]];`, it should equal to `vec![vec![1, 2], vec![3, 4]]`
#[macro_export]
macro_rules! vecc {
    (@array ($($vecs:expr,)*)) => {
        vec![$($vecs,)*]
    };

    (@array ($($vecs:expr,)*) [$($elem:expr),+] $($rest:tt)*) => {
        vecc!(@array ($($vecs,)* vec![$($elem),+],) $($rest)*)
    };

    (@array ($($vecs:expr,)*) [$($elem:expr,)*] $($rest:tt)*) => {
        vecc!(@array ($($vecs,)* vec![$($elem),*],) $($rest)*)
    };

    (@array ($($vecs:expr,)*) [$elem:expr; $n:expr] $($rest:tt)*) => {
        vecc!(@array ($($vecs,)* vec![$elem; $n],) $($rest)*)
    };

    [] => {
        vec![]
    };

    [ $($tt:tt,)+ ] => {
        vecc!(@array () $($tt)+)
    };

    [ $($tt:tt),+ ] => {
        vecc!(@array () $($tt)+)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vecc() {
        assert_eq!(vecc![[1, 2], [3, 4]], vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(vecc![[1, 2]], vec![vec![1, 2]]);
        assert_eq!(vecc![[1], [3]], vec![vec![1], vec![3]]);
        let _empty_vec: Vec<Vec<()>> = vecc![];
        assert_eq!(
            vecc![[1, 2, 3], [4, 5], []],
            vec![vec![1, 2, 3], vec![4, 5], vec![]]
        );
        assert_eq!(vecc![[()], [()]], vec![vec![()], vec![()]]);

        assert_eq!(vecc![[1, 2], [3, 4],], vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(vecc![[1, 2], [3, 4,],], vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(vecc![[1, 2]], vec![vec![1, 2]]);
        assert_eq!(vecc![[1; 2]], vec![vec![1; 2]]);
    }
}
