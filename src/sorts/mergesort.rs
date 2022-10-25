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
    merge(slice);
}

fn merge<'a, T: Ord + Debug + Copy + Clone>(slice: &mut [T]) {
    let middle = slice.len() / 2;
    let mut aux = Vec::<T>::new();
    let (mut li, mut ri) = (0, middle);
    while li < middle && ri < slice.len() {
        if slice[li] <= slice[ri] {
            aux.push(slice[li]);
            li += 1;
        } else {
            aux.push(slice[ri]);
            ri += 1;
        }
    }
    while li < middle {
        aux.push(slice[li]);
        li += 1;
    }
    while ri < slice.len() {
        aux.push(slice[ri]);
        ri += 1;
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
