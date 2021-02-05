# partition

partition slices in-place by a predicate

[![Build Status](https://travis-ci.org/llogiq/partition.svg)](https://travis-ci.org/llogiq/partition)
[![Current Version](http://meritbadge.herokuapp.com/bytecount)](https://crates.io/crates/partition)
[![License: Apache 2.0/MIT](https://img.shields.io/crates/l/bytecount.svg)](#license)

Rust has the `Iterator::partition(_)` method, but this requires allocating two
`Vec`s to hold the values. This has the benefit of preserving order, at the
cost of spending time and RAM to do the allocation, which also makes this API
unavailable on systems without an allocator.

This crate has a `partition(..)` function to partition a mutable slice of
values by a predicate. It will swap around values to separate those the
predicate holds for and those it doesn't and return the two mutable sub-slices.

This crate also provides `partition_index(..)` which operates similarly, but
instead returns the index of the first element to evaluate to false.  All
elements before this index evaluate to true, and all elements after evaluate
to false.

### Warning

Note that since partition works by swapping values, the order of elements within
the slice will not be preserved.

## Example

```Rust
let mut even_odd = [0u8, 1, 2, 3, 4, 5, 6];
let (even, odd) = partition(&mut even_odd, |x| x & 1 == 0);
```

## Performance

On a Core m3-6y30 with 4GB of RAM, I get the following benchmark results
(in ns/iter):

|Number of Elements|`partition(&[T], _)`|`Iter::partition(_)`|
|------------------|--------------------|--------------------|
|10000             |        8,738 ± 665 |   101,625 ± 10,930 |
|1000              |          896 ±  75 |     6,826 ±    760 |
|100               |          118 ±  11 |     1,013 ±    116 |
|10                |           14 ±   2 |       295 ±     93 |
|1                 |            4 ±   1 |        51 ±      7 |

So it's safe to say performance compares very favorably.

## License

Licensed under either of at your discretion:

- [Apache 2.0](LICENSE.Apache2)
- [MIT](LICENSE.MIT)
