use super::Sorter;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i] < slice[i - 1] {
                    slice.swap(i, i - 1);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn bubble_sort_works() {
    BubbleSort::sort::<i32>(&mut []); // test empty
    let mut items = vec![4, 2, 3, 1, 5];
    BubbleSort::sort(&mut items);
    assert_eq!(items, &[1, 2, 3, 4, 5]);
}
