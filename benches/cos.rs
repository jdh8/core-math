mod bench;

bench!(bench_core_math, core_math::cos, _);
bench!(bench_std, f64::cos, _);
bench!(bench_libm, libm::cos, _);

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
