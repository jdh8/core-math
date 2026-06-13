mod bench;

bench!(bench_core_math, core_math::log1pf, _);
bench!(bench_std, f32::ln_1p, _);
bench!(bench_libm, libm::log1pf, _);

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
