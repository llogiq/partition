extern crate bencher;
extern crate rand;
extern crate partition;

use rand::Rng;
use bencher::Bencher;
use partition::partition;

fn random_values(len: usize) -> Vec<u32> {
    rand::thread_rng().gen_iter::<u32>().take(len).collect();
}

fn true_fn(_: &u32) -> bool { true }
fn false_fn(_: &u32) -> bool { false }
fn odd_fn(x: &u32) -> bool { x % 2 > 0 }

macro_rules! mkbench {
    ($i: expr, $name_slice: ident, $name_vec: ident, $name_fn: ident) => {
        fn $name_slice(b: &mut Bencher) {
            let mut data = random_values($i);
            b.iter(|| partition(&mut data, $name_fn));
        }

        fn $name_vec(b: &mut Bencher) {
            let mut data = random_values($i);
            b.iter(|| data.iter().partition($name_fn));
        }
    }
}

mkbench!(1, bench_slice_1_true, bench_vec_1_true, true);
mkbench!(10, bench_slice_10_true, bench_vec_10_true, true);
mkbench!(100, bench_slice_100_true, bench_vec_100_true, true);
mkbench!(1000, bench_slice_1000_true, bench_vec_1000_true, true);
mkbench!(10000, bench_slice_10000_true, bench_vec_10000_true, true);

mkbench!(1, bench_slice_1_false, bench_vec_1_false, false);
mkbench!(10, bench_slice_10_false, bench_vec_10_false, false);
mkbench!(100, bench_slice_100_false, bench_vec_100_false, false);
mkbench!(1000, bench_slice_1000_false, bench_vec_1000_false, false);
mkbench!(10000, bench_slice_10000_false, bench_vec_10000_false, false);

mkbench!(1, bench_slice_1_odd, bench_vec_1_odd, odd);
mkbench!(10, bench_slice_10_odd, bench_vec_10_odd, odd);
mkbench!(100, bench_slice_100_odd, bench_vec_100_odd, odd);
mkbench!(1000, bench_slice_1000_odd, bench_vec_1000_odd, odd);
mkbench!(10000, bench_slice_10000_odd, bench_vec_10000_odd, odd);


benchmark_group!(bench,
    bench_slice_1_true, bench_slice_1_false, bench_slice_1_odd,
    bench_vec_1_true, bench_vec_1_false, bench_vec_1_odd,
    bench_slice_10_true, bench_slice_10_false, bench_slice_10_odd,
    bench_vec_10_true, bench_vec_10_false, bench_vec_10_odd,
    bench_slice_100_true, bench_slice_100_false, bench_slice_100_odd,
    bench_vec_100_true, bench_vec_100_false, bench_vec_100_odd,
    bench_slice_1000_true, bench_slice_1000_false, bench_slice_1000_odd,
    bench_vec_1000_true, bench_vec_1000_false, bench_vec_1000_odd,
    bench_slice_10000_true, bench_slice_10000_false, bench_slice_10000_odd,
    bench_vec_10000_true, bench_vec_10000_false, bench_vec_10000_odd,
);

benchmark_main!(bench);
