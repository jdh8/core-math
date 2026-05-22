mod bench;

bench!(bench_core_math, core_math::asinhf, _);
bench!(bench_std, f32::asinh, _);
bench!(bench_libm, libm::asinhf, _);

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
