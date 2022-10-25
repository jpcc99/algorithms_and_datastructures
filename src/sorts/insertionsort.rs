use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        let smart = false;
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            // takes slice[unsorted] and place localtion in slice[..=unsorted]
            // [ 1 3 4 | 2 ]
            // [ 1 3 4 2 | ]
            // [ 1 3 2 4 | ]
            // [ 1 3 2 4 | ]
            // [ 1 2 3 4 | ]
            if !smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                // use binary search to find index
                // then use .insert to splice it in i
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    // [a, c, e].binary_search(c) => Ok(1)
                    Ok(i) => i,
                    // [a, c, e].binary_search(b) => Err(1)
                    Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn works_with_empty_values() {
    let mut empty_vec = Vec::<i32>::new();
    InsertionSort::sort(&mut empty_vec);
}

#[test]
fn works_with_same_values() {
    let mut same_values = vec![1; 30];
    InsertionSort::sort(&mut same_values);
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
    InsertionSort::sort(&mut randon_num);
    for i in 1..randon_num.len() {
        if randon_num[i] < randon_num[i - 1] {
            is_sorted = false;
            break;
        }
    }
    assert!(is_sorted);
}
