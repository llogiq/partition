#[macro_use] extern crate bencher;
extern crate rand;
extern crate partition;

use rand::Rng;
use bencher::Bencher;
use partition::{partition, partition_index};

fn random_values(len: usize) -> Vec<u32> {
    rand::thread_rng().gen_iter::<u32>().take(len).collect()
}

fn true_fn(_: &u32) -> bool { true }
fn false_fn(_: &u32) -> bool { false }
fn odd_fn(x: &u32) -> bool { x % 2 > 0 }

macro_rules! mkbench {
    ($i: expr, $bench_name: ident, $name_fn: ident) => {
        mod $bench_name {
            use super::*;

            pub fn slice(b: &mut Bencher) {
                let mut data = random_values($i);
                b.iter(|| { let (_left, _right) = bencher::black_box(partition(&mut data, |e| $name_fn(e))); });
            }

            pub fn index(b: &mut Bencher) {
                let mut data = random_values($i);
                b.iter(|| { let _idx = bencher::black_box(partition_index(&mut data, |e| $name_fn(e))); });
            }

            pub fn vec(b: &mut Bencher) {
                let data : Vec<u32> = random_values($i);
                b.iter(|| data.iter().partition::<Vec<u32>, _>(|e| $name_fn(e)));
            }
        }
    }
}

mkbench!(1, bench_1_true, true_fn);
mkbench!(10, bench_10_true, true_fn);
mkbench!(100, bench_100_true, true_fn);
mkbench!(1000, bench_1000_true, true_fn);
mkbench!(10_000, bench_10000_true, true_fn);

mkbench!(1, bench_1_false, false_fn);
mkbench!(10, bench_10_false, false_fn);
mkbench!(100, bench_100_false, false_fn);
mkbench!(1000, bench_1000_false, false_fn);
mkbench!(10_000, bench_10000_false, false_fn);

mkbench!(1, bench_1_odd, odd_fn);
mkbench!(10, bench_10_odd, odd_fn);
mkbench!(100, bench_100_odd, odd_fn);
mkbench!(1000, bench_1000_odd, odd_fn);
mkbench!(10_000, bench_10000_odd, odd_fn);


benchmark_group!(bench,
    bench_1_true::slice, bench_1_false::slice, bench_1_odd::slice,
    bench_1_true::index, bench_1_false::index, bench_1_odd::index,
    bench_1_true::vec,   bench_1_false::vec,   bench_1_odd::vec,
    bench_10_true::slice, bench_10_false::slice, bench_10_odd::slice,
    bench_10_true::index, bench_10_false::index, bench_10_odd::index,
    bench_10_true::vec,   bench_10_false::vec,   bench_10_odd::vec,
    bench_100_true::slice, bench_100_false::slice, bench_100_odd::slice,
    bench_100_true::index, bench_100_false::index, bench_100_odd::index,
    bench_100_true::vec,   bench_100_false::vec,   bench_100_odd::vec,
    bench_1000_true::slice, bench_1000_false::slice, bench_1000_odd::slice,
    bench_1000_true::index, bench_1000_false::index, bench_1000_odd::index,
    bench_1000_true::vec,   bench_1000_false::vec,   bench_1000_odd::vec,
    bench_10000_true::slice, bench_10000_false::slice, bench_10000_odd::slice,
    bench_10000_true::index, bench_10000_false::index, bench_10000_odd::index,
    bench_10000_true::vec,   bench_10000_false::vec,   bench_10000_odd::vec,
);

benchmark_main!(bench);
