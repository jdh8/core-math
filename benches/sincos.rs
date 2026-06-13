mod bench;

bench!(bench_core_math, core_math::sincos, _);
bench!(bench_std, f64::sin_cos, _);
bench!(bench_libm, libm::sincos, _);

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
