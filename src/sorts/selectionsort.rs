use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
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
fn works_with_empty_values() {
    let mut empty_vec = Vec::<i32>::new();
    SelectionSort::sort(&mut empty_vec);
}

#[test]
fn works_with_same_values() {
    let mut same_values = vec![1; 30];
    SelectionSort::sort(&mut same_values);
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
    SelectionSort::sort(&mut randon_num);
    for i in 1..randon_num.len() {
        if randon_num[i] < randon_num[i - 1] {
            is_sorted = false;
            break;
        }
    }
    assert!(is_sorted);
}
