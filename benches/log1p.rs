mod bench;

bench!(bench_core_math, core_math::log1p, _);
bench!(bench_std, f64::ln_1p, _);
bench!(bench_libm, libm::log1p, _);

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
