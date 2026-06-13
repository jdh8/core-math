mod bench;

bench!(bench_core_math, core_math::acosf, in -1.1..1.1);
bench!(bench_std, f32::acos, in -1.1..1.1);
bench!(bench_libm, libm::acosf, in -1.1..1.1);

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
