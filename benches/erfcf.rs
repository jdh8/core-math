mod bench;

bench!(bench_core_math, core_math::erfcf, _);
bench!(bench_libm, libm::erfcf, _);

criterion::criterion_group!(benches, bench_core_math, bench_libm);
criterion::criterion_main!(benches);
