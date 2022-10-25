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
    aux
}

#[test]
fn works_with_empty_values() {
    let mut empty_vec = Vec::<i32>::new();
    MergeSort::sort(&mut empty_vec);
}

#[test]
fn works_with_same_values() {
    let mut same_values = vec![1; 30];
    MergeSort::sort(&mut same_values);
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
    let mut random_num = Vec::<i32>::with_capacity(CAPACITY);
    let mut rng = rand::thread_rng();
    let mut is_sorted = true;
    for num in random_num.iter_mut() {
        *num = rng.gen();
    }
    MergeSort::sort(&mut random_num);
    for i in 1..random_num.len() {
        if random_num[i] < random_num[i - 1] {
            is_sorted = false;
            break;
        }
    }
    assert!(is_sorted);
}
