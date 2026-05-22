mod bench;

bench!(bench_core_math, core_math::atan2f, _, _);
bench!(bench_std, f32::atan2, _, _);
bench!(bench_libm, libm::atan2f, _, _);

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
