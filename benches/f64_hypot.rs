mod bench;

bench!(bench_core_math, core_math::hypot, _, _);
bench!(bench_std, f64::hypot, _, _);
bench!(bench_libm, libm::hypot, _, _);

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
