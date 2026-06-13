#[macro_export]
macro_rules! bench {
    // The `_` arms must come before the generic arm because `expr` fragments
    // also match `_` since edition 2024
    ($name:ident, $callback:expr, _ $(,)?) => {
        bench!($name, $callback, rand::random());
    };
    ($name:ident, $callback:expr, _, _ $(,)?) => {
        bench!($name, $callback, rand::random(), rand::random());
    };
    ($name:ident, $callback:expr, in $range:expr $(,)?) => {
        bench!($name, $callback, rand::random_range($range));
    };
    ($name:ident, $callback:expr, _, in $range:expr $(,)?) => {
        bench!($name, $callback, rand::random(), rand::random_range($range));
    };
    ($name:ident, $callback:expr $(, $args:expr)* $(,)?) => {
        fn $name(criterion: &mut criterion::Criterion) {
            criterion.bench_function(stringify!($callback), |bencher| {
                bencher.iter(|| $callback($($args),*));
            });
        }
    };
}
