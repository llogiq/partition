//! This crate has only one `partition(&[T], P)` function to partition slices
//! in place.
#![cfg_attr(not(test), no_std)]

#[cfg(test)]
extern crate quickcheck;

/// partition a mutable slice in-place so that it contains all elements for
/// which `predicate(e)` is `true`, followed by all elements for which
/// `predicate(e)` is `false`. Returns sub-slices to all predicated and
/// non-predicated elements, respectively.
///
/// This does roughly the same as `Iterator::partition(_)`, but without
/// requiring any additional storage.
///
/// Examples
///
/// ```
///# use partition::partition;
/// let mut even_odd = [0u8, 1, 2, 3, 4, 5, 6];
/// let (even, odd) = partition(&mut even_odd, |x| x & 1 == 0);
/// assert!(&[0, 2, 4, 6].iter().all(|x| even.iter().any(|e| e == x)), "expected [0, 2, 4, 6], got {:?}", even);
/// assert!(&[1, 3, 5].iter().all(|x| odd.iter().any(|o| o == x)));
/// ```
pub fn partition<T, P>(data: &mut [T], predicate: P) -> (&mut [T], &mut [T])
where P: Fn(&T) -> bool {
    let len = data.len();
    if len == 0 { return (&mut [], &mut []); }
    let (mut l, mut r) = (0, len - 1);
    loop {
        while l < len && predicate(&data[l]) { l += 1; }
        while r > 0 && !predicate(&data[r]) { r -= 1; }
        if l >= r { return data.split_at_mut(l); }
        data.swap(l, r);
    }
}

/// partition a mutable slice in-place so that it contains all elements for
/// which `predicate(e)` is `true`, followed by all elements for which
/// `predicate(e)` is `false`.
/// Returns the index of the first element which returned false.
/// Returns 0 if all elements returned false.
/// Returns `data.len()` if all elements returned true.
///
/// Examples
///
/// ```
///# use partition::partition_index;
/// let mut even_odd = [0u8, 1, 2, 3, 4, 5, 6];
/// let first_odd = partition_index(&mut even_odd, |x| x & 1 == 0);
/// assert_eq!(first_odd, 4, "expected an index of 4, got {:?}", first_odd);
/// for (idx, &e) in even_odd.iter().enumerate() {
///   if idx < first_odd {
///     assert!(e & 1 == 0, "expected elements before first_odd to be even, found {:?}", e);
///   } else {
///     assert!(e & 1 == 1, "expected elements after first_odd to be odd, found {:?}", e);
///   } 
/// }
pub fn partition_index<T, P>(data: &mut [T], predicate: P) -> usize
where P: Fn(&T) -> bool {
    let len = data.len();
    if len == 0 { return 0; }
    let (mut l, mut r) = (0, len - 1);
    loop {
        while l < len && predicate(&data[l]) { l += 1; }
        while r > 0 && !predicate(&data[r]) { r -= 1; }
        if l >= r { return l; }
        data.swap(l, r);
    }
}


#[cfg(test)]
mod tests {
    use super::{partition, partition_index};
    use quickcheck::QuickCheck;

    #[test]
    fn test_empty() {
        let empty : &mut [usize] = &mut [][..];
        assert_eq!((&mut [][..], &mut [][..]), partition(empty, |_| true));
        assert_eq!(0, partition_index(empty, |_| true));
    }

    #[test]
    fn test_single_true() {
        assert_eq!((&mut [1][..], &mut [][..]), partition(&mut [1u8][..], |_| true));
        assert_eq!(1, partition_index(&mut [1u8][..], |_| true));
    }

    #[test]
    fn test_single_false() {
        assert_eq!((&mut [][..], &mut [1][..]), partition(&mut [1u8][..], |_| false));
        assert_eq!(0, partition_index(&mut [1u8][..], |_| false));
    }

    #[test]
    fn quickcheck() {
        fn prop_partition(data: Vec<u32>) -> bool {
            let mut data = data;
            let mut trues = data.iter().cloned().filter(|e| e % 2 == 0).collect::<Vec<u32>>();
            let mut falses = data.iter().cloned().filter(|e| e % 2 != 0).collect::<Vec<u32>>();
            let (left, right) = partition(&mut data, |&e| e % 2 == 0);
            trues.sort();
            falses.sort();
            left.sort();
            right.sort();
            trues == left && falses == right
        }

        fn prop_partition_index(data: Vec<u32>) -> bool {
            let mut data = data;
            let mut trues = data.iter().cloned().filter(|e| e % 2 == 0).collect::<Vec<u32>>();
            let mut falses = data.iter().cloned().filter(|e| e % 2 != 0).collect::<Vec<u32>>();
            let first_false = partition_index(&mut data, |&e| e % 2 == 0);
            let (mut left, mut right) = (
                data[0..first_false].iter().cloned().collect::<Vec<u32>>(),
                data[first_false..].iter().cloned().collect::<Vec<u32>>()
            );
            trues.sort();
            falses.sort();
            left.sort();
            right.sort();
            trues == left && falses == right
        }
        QuickCheck::new().tests(10000).quickcheck(prop_partition as fn(Vec<u32>) -> bool);
        QuickCheck::new().tests(10000).quickcheck(prop_partition_index as fn(Vec<u32>) -> bool);
    }
}
