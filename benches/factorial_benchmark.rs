//! Criterion has become the defacto standard for benchmarking rust code.
//! https://bheisler.github.io/criterion.rs/book/criterion_rs.html
//!
//! run `cargo bench` and then open `target/criterion/report/index.html` to see all that nerdy goodness

use testing_rust::factorial;

use criterion::{Criterion, BenchmarkId, black_box, criterion_group, criterion_main};


pub fn benchmark_factorial(crit: &mut Criterion) {
    let mut group = crit.benchmark_group("factorial");
    // this is a simple way to benchmark a function with a variety of inputs
    for input in [1, 5, 20, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(input), input, |bencher, &num| {
            // there's lots of different functions we could call on `bencher` to support
            // different use cases. `iter` is the simplest and works for most things.
            bencher.iter(|| {
                // anything inside this function body is going to get timed, so we really
                // don't want to add any correctness assertions here.
                // Also note the use of `black_box` (https://docs.rs/criterion/0.3.0/criterion/fn.black_box.html),
                // which ensures that this call doesn't get optimized away by the compiler. If your
                // benchmarks seem spooky fast, be suspicious.
                factorial(black_box(num))
            });
        });
    }
}

// Ok this stuff is a little weird. Since criterion doesn't use the built-in benchmarking harness
// we need some way to setup a main function that serves as the entrypoint to the benchmark executable.
// This is what these two macros do. `criterion_group` allows you to group benchmarks, and you pass
// the list of all the groups to `criterion_main`
criterion_group!(static_benchmarks, benchmark_factorial);
criterion_main!(static_benchmarks);
