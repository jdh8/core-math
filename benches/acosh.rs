mod bench;

bench!(
    bench_core_math,
    core_math::acosh,
    rand::random::<f64>().abs()
);

bench!(bench_std, f64::acosh, rand::random::<f64>().abs());
bench!(bench_libm, libm::acosh, rand::random::<f64>().abs());

criterion::criterion_group!(benches, bench_core_math, bench_std, bench_libm);
criterion::criterion_main!(benches);
