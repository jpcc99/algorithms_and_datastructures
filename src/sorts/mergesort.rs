use std::fmt::Debug;

use super::Sorter;

pub struct MergeSort;

impl Sorter for MergeSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord + Debug + Copy + Clone,
    {
        mergesort(slice);
    }
}

fn mergesort<T: Ord + Debug + Copy + Clone>(slice: &mut [T]) {
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
        slice[i] = *v;
    }
}

fn merge<'a, T: Ord + Debug + Copy + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut aux = Vec::<T>::with_capacity(left.len() + right.len());
    let (mut left, mut right) = (left.iter().peekable(), right.iter().peekable());
    while let (Some(lv), Some(rv)) = (left.peek(), right.peek()) {
        if lv <= rv {
            left.next().take().map(|v| aux.push(*v));
        } else {
            right.next().take().map(|v| aux.push(*v));
        }
    }
    for lv in left {
        aux.push(*lv);
    }
    for rv in right {
        aux.push(*rv);
    }
    for (i, v) in aux.iter().enumerate() {
        slice[i] = *v;
    }
}

#[test]
fn mergesort_works() {
    let mut items = vec![0, 4, 2, 3, 81, 1, 7, 31];
    println!("{:?}", items);
    MergeSort::sort(&mut items);
    assert_eq!(items, &[0, 1, 2, 3, 4, 7, 31, 81]);
}
