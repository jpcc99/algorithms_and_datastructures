use super::Sorter;

pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord
    {
        // [ unsorted | pivot | unsorted ]
        quicksort(slice);
    }
}

fn quicksort<T: Ord>(slice: &mut [T]) {
    if slice.is_empty() || slice.len() < 2 {
        return;
    } else if slice.len() == 2 { 
        if slice[0] > slice[1] {
            slice.swap(0, 1);
        }
        return;
    }
    let pp = get_partition_point(slice);
    let (left, right) = slice.split_at_mut(pp);
    quicksort(left);
    quicksort(&mut right[1..]);
}

fn get_partition_point<T: Ord>(slice: &mut [T]) -> usize {
    let (pivot, rest) = slice.split_last_mut().expect("array not empty");
    let mut i = 0;
    for j in 0..rest.len() {
        if &rest[j] < pivot {
            rest.swap(i, j);
            i += 1;
        }
    }
    if slice[i] > slice[slice.len() - 1] {
        slice.swap(i, slice.len() - 1);
    }
    i
}

#[test]
fn check_quiicksort() {
    let mut items = vec![0, 4, 2, 3, 81, 1, 7, 31];
    println!("{:?}", items);
    QuickSort::sort(&mut items);
    assert_eq!(items, &[0, 1, 2, 3, 4, 7, 31, 81]);
}
