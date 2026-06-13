mod bench;

bench!(bench_core_math, core_math::log, _);
bench!(bench_std, f64::ln, _);
bench!(bench_libm, libm::log, _);

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
