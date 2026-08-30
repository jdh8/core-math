//! Every CORE-MATH function against `std` and `libm`, in one criterion target.
//!
//! Run a subset with criterion's own filter, e.g. `cargo bench -- core_math::acos`.

fn all(c: &mut criterion::Criterion) {
    macro_rules! bench {
        ($f:expr $(, $arg:expr)* $(,)?) => {
            c.bench_function(stringify!($f), |b| b.iter(|| $f($($arg),*)));
        };
    }

    bench!(core_math::acos, rand::random_range(-1.1..1.1));
    bench!(f64::acos, rand::random_range(-1.1..1.1));
    bench!(libm::acos, rand::random_range(-1.1..1.1));

    bench!(core_math::acosf, rand::random_range(-1.1..1.1));
    bench!(f32::acos, rand::random_range(-1.1..1.1));
    bench!(libm::acosf, rand::random_range(-1.1..1.1));

    bench!(core_math::acosh, rand::random::<f64>().abs());
    bench!(f64::acosh, rand::random::<f64>().abs());
    bench!(libm::acosh, rand::random::<f64>().abs());

    bench!(core_math::acoshf, rand::random::<f32>().abs());
    bench!(f32::acosh, rand::random::<f32>().abs());
    bench!(libm::acoshf, rand::random::<f32>().abs());

    bench!(core_math::asin, rand::random_range(-1.1..1.1));
    bench!(f64::asin, rand::random_range(-1.1..1.1));
    bench!(libm::asin, rand::random_range(-1.1..1.1));

    bench!(core_math::asinf, rand::random_range(-1.1..1.1));
    bench!(f32::asin, rand::random_range(-1.1..1.1));
    bench!(libm::asinf, rand::random_range(-1.1..1.1));

    bench!(core_math::asinh, rand::random());
    bench!(f64::asinh, rand::random());
    bench!(libm::asinh, rand::random());

    bench!(core_math::asinhf, rand::random());
    bench!(f32::asinh, rand::random());
    bench!(libm::asinhf, rand::random());

    bench!(core_math::atan, rand::random());
    bench!(f64::atan, rand::random());
    bench!(libm::atan, rand::random());

    bench!(core_math::atanf, rand::random());
    bench!(f32::atan, rand::random());
    bench!(libm::atanf, rand::random());

    bench!(core_math::atan2, rand::random(), rand::random());
    bench!(f64::atan2, rand::random(), rand::random());
    bench!(libm::atan2, rand::random(), rand::random());

    bench!(core_math::atan2f, rand::random(), rand::random());
    bench!(f32::atan2, rand::random(), rand::random());
    bench!(libm::atan2f, rand::random(), rand::random());

    bench!(core_math::atanh, rand::random_range(-1.1..1.1));
    bench!(f64::atanh, rand::random_range(-1.1..1.1));
    bench!(libm::atanh, rand::random_range(-1.1..1.1));

    bench!(core_math::atanhf, rand::random_range(-1.1..1.1));
    bench!(f32::atanh, rand::random_range(-1.1..1.1));
    bench!(libm::atanhf, rand::random_range(-1.1..1.1));

    bench!(core_math::cbrt, rand::random());
    bench!(f64::cbrt, rand::random());
    bench!(libm::cbrt, rand::random());

    bench!(core_math::cbrtf, rand::random());
    bench!(f32::cbrt, rand::random());
    bench!(libm::cbrtf, rand::random());

    bench!(core_math::compoundf, rand::random(), rand::random());

    bench!(core_math::cos, rand::random());
    bench!(f64::cos, rand::random());
    bench!(libm::cos, rand::random());

    bench!(core_math::cosf, rand::random());
    bench!(f32::cos, rand::random());
    bench!(libm::cosf, rand::random());

    bench!(core_math::cosh, rand::random());
    bench!(f64::cosh, rand::random());
    bench!(libm::cosh, rand::random());

    bench!(core_math::coshf, rand::random());
    bench!(f32::cosh, rand::random());
    bench!(libm::coshf, rand::random());

    bench!(core_math::erf, rand::random());
    bench!(libm::erf, rand::random());

    bench!(core_math::erff, rand::random());
    bench!(libm::erff, rand::random());

    bench!(core_math::erfc, rand::random());
    bench!(libm::erfc, rand::random());

    bench!(core_math::erfcf, rand::random());
    bench!(libm::erfcf, rand::random());

    bench!(core_math::exp, rand::random_range(-745.0..710.0));
    bench!(f64::exp, rand::random_range(-745.0..710.0));
    bench!(libm::exp, rand::random_range(-745.0..710.0));

    bench!(core_math::expf, rand::random_range(-105.0..90.0));
    bench!(f32::exp, rand::random_range(-105.0..90.0));
    bench!(libm::expf, rand::random_range(-105.0..90.0));

    bench!(core_math::exp10, rand::random_range(-325.0..310.0));
    bench!(libm::exp10, rand::random_range(-325.0..310.0));

    bench!(core_math::exp10f, rand::random_range(-50.0..40.0));
    bench!(libm::exp10f, rand::random_range(-50.0..40.0));

    bench!(core_math::exp2, rand::random_range(-1075.0..1025.0));
    bench!(f64::exp2, rand::random_range(-1075.0..1025.0));
    bench!(libm::exp2, rand::random_range(-1075.0..1025.0));

    bench!(core_math::exp2f, rand::random_range(-155.0..130.0));
    bench!(f32::exp2, rand::random_range(-155.0..130.0));
    bench!(libm::exp2f, rand::random_range(-155.0..130.0));

    bench!(core_math::expm1, rand::random_range(-40.0..710.0));
    bench!(f64::exp_m1, rand::random_range(-40.0..710.0));
    bench!(libm::expm1, rand::random_range(-40.0..710.0));

    bench!(core_math::expm1f, rand::random_range(-20.0..90.0));
    bench!(f32::exp_m1, rand::random_range(-20.0..90.0));
    bench!(libm::expm1f, rand::random_range(-20.0..90.0));

    bench!(core_math::hypot, rand::random(), rand::random());
    bench!(f64::hypot, rand::random(), rand::random());
    bench!(libm::hypot, rand::random(), rand::random());

    bench!(core_math::hypotf, rand::random(), rand::random());
    bench!(f32::hypot, rand::random(), rand::random());
    bench!(libm::hypotf, rand::random(), rand::random());

    bench!(core_math::lgamma, rand::random());
    bench!(libm::lgamma, rand::random());

    bench!(core_math::lgammaf, rand::random());
    bench!(libm::lgammaf, rand::random());

    bench!(core_math::log, rand::random());
    bench!(f64::ln, rand::random());
    bench!(libm::log, rand::random());

    bench!(core_math::logf, rand::random());
    bench!(f32::ln, rand::random());
    bench!(libm::logf, rand::random());

    bench!(core_math::log10, rand::random());
    bench!(f64::log10, rand::random());
    bench!(libm::log10, rand::random());

    bench!(core_math::log10f, rand::random());
    bench!(f32::log10, rand::random());
    bench!(libm::log10f, rand::random());

    bench!(core_math::log1p, rand::random());
    bench!(f64::ln_1p, rand::random());
    bench!(libm::log1p, rand::random());

    bench!(core_math::log1pf, rand::random());
    bench!(f32::ln_1p, rand::random());
    bench!(libm::log1pf, rand::random());

    bench!(core_math::log2, rand::random());
    bench!(f64::log2, rand::random());
    bench!(libm::log2, rand::random());

    bench!(core_math::log2f, rand::random());
    bench!(f32::log2, rand::random());
    bench!(libm::log2f, rand::random());

    bench!(core_math::pow, rand::random(), rand::random());
    bench!(f64::powf, rand::random(), rand::random());
    bench!(libm::pow, rand::random(), rand::random());

    bench!(core_math::powf, rand::random(), rand::random());
    bench!(f32::powf, rand::random(), rand::random());
    bench!(libm::powf, rand::random(), rand::random());

    bench!(core_math::sin, rand::random());
    bench!(f64::sin, rand::random());
    bench!(libm::sin, rand::random());

    bench!(core_math::sinf, rand::random());
    bench!(f32::sin, rand::random());
    bench!(libm::sinf, rand::random());

    bench!(core_math::sincos, rand::random());
    bench!(f64::sin_cos, rand::random());
    bench!(libm::sincos, rand::random());

    bench!(core_math::sincosf, rand::random());
    bench!(f32::sin_cos, rand::random());
    bench!(libm::sincosf, rand::random());

    bench!(core_math::sinh, rand::random());
    bench!(f64::sinh, rand::random());
    bench!(libm::sinh, rand::random());

    bench!(core_math::sinhf, rand::random());
    bench!(f32::sinh, rand::random());
    bench!(libm::sinhf, rand::random());

    bench!(core_math::tan, rand::random());
    bench!(f64::tan, rand::random());
    bench!(libm::tan, rand::random());

    bench!(core_math::tanf, rand::random());
    bench!(f32::tan, rand::random());
    bench!(libm::tanf, rand::random());

    bench!(core_math::tanh, rand::random());
    bench!(f64::tanh, rand::random());
    bench!(libm::tanh, rand::random());

    bench!(core_math::tanhf, rand::random());
    bench!(f32::tanh, rand::random());
    bench!(libm::tanhf, rand::random());

    bench!(core_math::tgamma, rand::random());
    bench!(libm::tgamma, rand::random());

    bench!(core_math::tgammaf, rand::random());
    bench!(libm::tgammaf, rand::random());
}

criterion::criterion_group!(benches, all);
criterion::criterion_main!(benches);
