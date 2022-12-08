# Haskell style QuickSort, but Rust version

[Haskell version](https://riptutorial.com/haskell/example/7553/quicksort):

```haskell
qsort :: (Ord a) => [a] -> [a]
qsort [] = []
qsort (x:xs) = qsort [a | a <- xs, a < x]
                      ++ [x] ++
               qsort [b | b <- xs, b >= x]
```

[Rust version](src/lib.rs)

```rust
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
```

**Just for fun, DO NOT USE it in any actually projects.**
