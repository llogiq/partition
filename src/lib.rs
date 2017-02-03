//! This crate has only one `partition(&[T], P)` function to partition slices
//! in place.
#[no_std]

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
pub fn partition<'a, T, P>(data: &mut [T], predicate: P) -> (&mut [T], &mut [T])
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

#[cfg(test)]
mod tests {
    use super::partition;
    use quickcheck::QuickCheck;

    #[test]
    fn test_empty() {
        let empty : &mut [usize] = &mut [][..];
        assert_eq!((&mut [][..], &mut [][..]), partition(empty, |_| true));
    }

    #[test]
    fn test_single_true() {
        assert_eq!((&mut [1][..], &mut [][..]), partition(&mut [1u8][..], |_| true));
    }

    #[test]
    fn test_single_false() {
        assert_eq!((&mut [][..], &mut [1][..]), partition(&mut [1u8][..], |_| false));
    }

    #[test]
    fn quickcheck() {
        fn prop(data: Vec<u32>) -> bool {
            let mut xs = data.clone();
            let mut trues = xs.iter().cloned().filter(|e| e % 2 == 0).collect::<Vec<u32>>();
            let mut falses = xs.iter().cloned().filter(|e| e % 2 != 0).collect::<Vec<u32>>();
            let (left, right) = partition(&mut xs, |&e| e % 2 == 0);
            trues.sort();
            falses.sort();
            left.sort();
            right.sort();
            trues == left && falses == right
        }
        QuickCheck::new().tests(10000).quickcheck(prop as fn(Vec<u32>) -> bool);
    }
}
