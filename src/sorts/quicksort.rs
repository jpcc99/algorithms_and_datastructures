use super::Sorter;

pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
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
fn works_with_empty_values() {
    let mut empty_vec = Vec::<i32>::new();
    QuickSort::sort(&mut empty_vec);
}

#[test]
fn works_with_same_values() {
    let mut same_values = vec![1; 30];
    QuickSort::sort(&mut same_values);
    let mut is_sorted = true;
    for i in 1..same_values.len() {
        if same_values[i] < same_values[i - 1] {
            is_sorted = false;
            break;
        }
    }
    assert!(is_sorted);
}

#[test]
fn works_with_random_values() {
    use rand::prelude::*;
    const CAPACITY: usize = 100000;
    let mut randon_num = Vec::<i32>::with_capacity(CAPACITY);
    let mut rng = rand::thread_rng();
    let mut is_sorted = true;
    for num in randon_num.iter_mut() {
        *num = rng.gen();
    }
    QuickSort::sort(&mut randon_num);
    for i in 1..randon_num.len() {
        if randon_num[i] < randon_num[i - 1] {
            is_sorted = false;
            break;
        }
    }
    assert!(is_sorted);
}
