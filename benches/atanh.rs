mod bench;

bench!(bench_core_math, core_math::atanh, in -1.1..1.1);
bench!(bench_std, f64::atanh, in -1.1..1.1);
bench!(bench_libm, libm::atanh, in -1.1..1.1);

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
