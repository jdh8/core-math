mod bench;

bench!(bench_core_math, core_math::exp10f, in -50.0..40.0);
bench!(bench_libm, libm::exp10f, in -50.0..40.0);

criterion::criterion_group!(benches, bench_core_math, bench_libm);
criterion::criterion_main!(benches);
