mod bench;

bench!(bench_core_math, core_math::tgammaf, _);
bench!(bench_libm, libm::tgammaf, _);

criterion::criterion_group!(benches, bench_core_math, bench_libm);
criterion::criterion_main!(benches);
