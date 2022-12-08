pub fn qsort<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    match list {
        [] => vec![],
        [x, xs @ ..] => {
            let (lesser, greater): (Vec<_>, Vec<_>) =
                xs.to_owned().into_iter().partition(|y| *y < *x);
            [qsort(&lesser), vec![x.to_owned()], qsort(&greater)].concat()
        }
    }
}

#[cfg(test)]
mod qsort_tests {
    use super::*;

    #[test]
    fn it_just_works() {
        let result = qsort(&[1, 2, 5, 6, 789213, 99, -293, 8, 1]);
        assert_eq!(result, [-293, 1, 1, 2, 5, 6, 8, 99, 789213]);
    }
}
