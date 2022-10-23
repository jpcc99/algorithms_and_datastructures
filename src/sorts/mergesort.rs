use super::Sorter;

pub struct MergeSort;

impl Sorter for MergeSort {
    fn sort<T>(slice: &mut [T]) where T: Ord {
        mergesort(slice);
    }
}

fn mergesort<T: Ord>(slice: &mut [T]) {
    if slice.is_empty() || slice.len() < 2 {
        return;
    } else if slice.len() == 2 && slice[0] > slice[1] {
        slice.swap(0, 1);
        return;
    }
    let middle: usize = slice.len() / 2;
    let (left, right) = slice.split_at_mut(middle);
    mergesort(left);
    mergesort(right);
    for (i, v) in merge(left, right).iter().enumerate() {
        if &i != v {
            slice.swap(i, *v);
        }
    }
}

fn merge<'a, T: Ord>(left: &'a mut [T], right: &'a mut [T]) -> Vec<usize> {
    todo!();
}

#[test]
fn mergesort_works() {
    let mut items = vec![0, 4, 2, 3, 81, 1, 7, 31];
    println!("{:?}", items);
    MergeSort::sort(&mut items);
    assert_eq!(items, &[0, 1, 2, 3, 4, 7, 31, 81]);
}
