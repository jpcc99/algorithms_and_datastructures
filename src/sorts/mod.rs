pub trait Sorter {
    fn sort<T>(slice: &mut [T]) where T: Ord;
}

pub fn sort<T, S>(slice: &mut [T]) 
    where
    T: Ord,
    S: Sorter,
{
    S::sort(slice)
}

pub trait StdSorter {
    fn sort<T>(slice: &mut [T]) where T: Ord + Copy + Clone;
}

mod bubblesort;
mod insertionsort;
mod selectionsort;
mod quicksort;
mod mergesort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use selectionsort::SelectionSort;
pub use quicksort::QuickSort;
pub use mergesort::MergeSort;

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T]) where T: Ord {
            slice.sort()
        }
    }

    #[test]
    fn std_works() {
        let mut items = vec![0, 4, 2, 3, 81, 1, 7, 31];
        StdSorter::sort(&mut items);
        assert_eq!(items, &[0, 1, 2, 3, 4, 7, 31, 81]);
    }
}
