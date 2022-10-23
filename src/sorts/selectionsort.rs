use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(slice: &mut [T]) where T: Ord {
        for unsorted in 0..slice.len() {
            let smallest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| i + unsorted)
                .expect("slice is not suppose to be empty");
            if smallest != unsorted {
                slice.swap(unsorted, smallest);
            }
        }
    }
}

#[test]
fn selectionsort() {
        let mut items = vec![0, 4, 2, 3, 81, 1, 7, 31];
        SelectionSort::sort(&mut items);
        assert_eq!(items, &[0, 1, 2, 3, 4, 7, 31, 81]);
}
