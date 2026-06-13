mod bench;

bench!(bench_core_math, core_math::exp, in -745.0..710.0);
bench!(bench_std, f64::exp, in -745.0..710.0);
bench!(bench_libm, libm::exp, in -745.0..710.0);

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
