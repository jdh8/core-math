mod bench;

bench!(bench_core_math, core_math::compoundf, _, _);

criterion::criterion_group!(benches, bench_core_math);
criterion::criterion_main!(benches);
