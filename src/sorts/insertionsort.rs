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
fn insertionsort_works() {
    InsertionSort::sort::<i32>(&mut []); // test empty
    let mut items = vec![4, 2, 3, 1, 5];
    InsertionSort::sort(&mut items);
    assert_eq!(items, &[1, 2, 3, 4, 5]);
}
