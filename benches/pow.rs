mod bench;

bench!(bench_core_math, core_math::pow, _, _);
bench!(bench_std, f64::powf, _, _);
bench!(bench_libm, libm::pow, _, _);

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
