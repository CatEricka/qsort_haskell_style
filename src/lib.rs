use std::cmp::Ordering;

/// Sort the slice, return `Vec<T>`
///
/// This sort is unstable.
///
/// # Example:
///
/// ```
/// use qsort_haskell_style::qsort;
/// let result = qsort(&[1, 2, 5, 6, 789213, 99, -293, 8, 1]);
/// assert_eq!(result, [-293, 1, 1, 2, 5, 6, 8, 99, 789213]);
/// ```
pub fn qsort<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    match list {
        [] => vec![],
        [x, xs @ ..] => {
            let (lesser, greater): (Vec<_>, Vec<_>) =
                xs.to_owned().into_iter().partition(|y| y.lt(x));
            [qsort(&lesser), vec![x.to_owned()], qsort(&greater)].concat()
        }
    }
}

/// Sorts the slice with a comparator function,
/// return `Vec<T>`
///
/// This sort is unstable.
///
/// # Example:
///
/// ```
/// use qsort_haskell_style::qsort_by;
/// let result = qsort_by(&[1, 2, 5, 6, 789213, 99, -293, 8, 1], |a, b| a.cmp(b));
/// assert_eq!(result, [-293, 1, 1, 2, 5, 6, 8, 99, 789213]);
/// ```
pub fn qsort_by<T: Clone, F>(list: &[T], compare: F) -> Vec<T>
where
    F: Fn(&T, &T) -> Ordering,
{
    qsort_by_internal(list, &compare)
}

fn qsort_by_internal<T: Clone, F>(list: &[T], compare: &F) -> Vec<T>
where
    F: Fn(&T, &T) -> Ordering,
{
    match list {
        [] => vec![],
        [x, xs @ ..] => {
            let (lesser, greater): (Vec<_>, Vec<_>) = xs
                .to_owned()
                .into_iter()
                .partition(|y| compare(y, x) == Ordering::Less);
            [
                qsort_by_internal(&lesser, compare),
                vec![x.to_owned()],
                qsort_by_internal(&greater, compare),
            ]
            .concat()
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

    #[test]
    fn qsort_by_asc() {
        let result = qsort_by(&[1, 2, 5, 6, 789213, 99, -293, 8, 1], |a, b| a.cmp(b));
        assert_eq!(result, [-293, 1, 1, 2, 5, 6, 8, 99, 789213]);
    }

    #[test]
    fn qsort_by_desc() {
        let result = qsort_by(&[20, 9, -2, 3, 9, 8], |a, b| b.cmp(a));
        assert_eq!(result, [20, 9, 9, 8, 3, -2]);
    }
}
