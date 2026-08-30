#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]

use core_math_sys as sys;

/// Wrap CORE-MATH functions whose arguments and result share one type.
///
/// `sincos` is the only shape that does not fit; it is written out by hand.
macro_rules! wrap {
    ($t:ty { $($f:ident = $sys:ident($($a:ident),+)),+ $(,)? }) => {$(
        #[must_use]
        #[inline]
        pub fn $f($($a: $t),+) -> $t {
            unsafe { sys::$sys($($a),+) }
        }
    )+};
}

/// Wrap a CORE-MATH `sincos` variant, which writes through two out-pointers.
macro_rules! wrap_sincos {
    ($t:ty, $f:ident = $sys:ident) => {
        #[must_use]
        #[inline]
        pub fn $f(x: $t) -> ($t, $t) {
            let (mut s, mut c) = (0.0, 0.0);
            unsafe { sys::$sys(x, &mut s, &mut c) };
            (s, c)
        }
    };
}

/////// `f32` functions ///////

wrap!(f32 {
    acosf = cr_acosf(x),
    acoshf = cr_acoshf(x),
    acospif = cr_acospif(x),
    asinf = cr_asinf(x),
    asinhf = cr_asinhf(x),
    asinpif = cr_asinpif(x),
    atanf = cr_atanf(x),
    atan2f = cr_atan2f(y, x),
    atan2pif = cr_atan2pif(y, x),
    atanhf = cr_atanhf(x),
    atanpif = cr_atanpif(x),
    cbrtf = cr_cbrtf(x),
    compoundf = cr_compoundf(x, y),
    cosf = cr_cosf(x),
    coshf = cr_coshf(x),
    cospif = cr_cospif(x),
    erff = cr_erff(x),
    erfcf = cr_erfcf(x),
    expf = cr_expf(x),
    exp10f = cr_exp10f(x),
    exp10m1f = cr_exp10m1f(x),
    exp2f = cr_exp2f(x),
    exp2m1f = cr_exp2m1f(x),
    expm1f = cr_expm1f(x),
    hypotf = cr_hypotf(x, y),
    lgammaf = cr_lgammaf(x),
    logf = cr_logf(x),
    log10f = cr_log10f(x),
    log10p1f = cr_log10p1f(x),
    log1pf = cr_log1pf(x),
    log2f = cr_log2f(x),
    log2p1f = cr_log2p1f(x),
    powf = cr_powf(x, y),
    rsqrtf = cr_rsqrtf(x),
    sinf = cr_sinf(x),
    sinhf = cr_sinhf(x),
    sinpif = cr_sinpif(x),
    tanf = cr_tanf(x),
    tanhf = cr_tanhf(x),
    tanpif = cr_tanpif(x),
    tgammaf = cr_tgammaf(x),
});

wrap_sincos!(f32, sincosf = cr_sincosf);

/////// `f64` functions ///////

wrap!(f64 {
    acos = cr_acos(x),
    acosh = cr_acosh(x),
    acospi = cr_acospi(x),
    asin = cr_asin(x),
    asinh = cr_asinh(x),
    asinpi = cr_asinpi(x),
    atan = cr_atan(x),
    atan2 = cr_atan2(y, x),
    atan2pi = cr_atan2pi(y, x),
    atanh = cr_atanh(x),
    atanpi = cr_atanpi(x),
    cbrt = cr_cbrt(x),
    cos = cr_cos(x),
    cosh = cr_cosh(x),
    cospi = cr_cospi(x),
    erf = cr_erf(x),
    erfc = cr_erfc(x),
    exp = cr_exp(x),
    exp10 = cr_exp10(x),
    exp10m1 = cr_exp10m1(x),
    exp2 = cr_exp2(x),
    exp2m1 = cr_exp2m1(x),
    expm1 = cr_expm1(x),
    hypot = cr_hypot(x, y),
    lgamma = cr_lgamma(x),
    log = cr_log(x),
    log10 = cr_log10(x),
    log10p1 = cr_log10p1(x),
    log1p = cr_log1p(x),
    log2 = cr_log2(x),
    log2p1 = cr_log2p1(x),
    pow = cr_pow(x, y),
    rsqrt = cr_rsqrt(x),
    sin = cr_sin(x),
    sinh = cr_sinh(x),
    sinpi = cr_sinpi(x),
    tan = cr_tan(x),
    tanh = cr_tanh(x),
    tanpi = cr_tanpi(x),
    tgamma = cr_tgamma(x),
});

wrap_sincos!(f64, sincos = cr_sincos);

/////// binary16 (`f16`) functions ///////

#[cfg(feature = "f16")]
wrap!(f16 {
    acosf16 = cr_acosf16(x),
    acoshf16 = cr_acoshf16(x),
    acospif16 = cr_acospif16(x),
    asinf16 = cr_asinf16(x),
    asinhf16 = cr_asinhf16(x),
    asinpif16 = cr_asinpif16(x),
    atanf16 = cr_atanf16(x),
    atan2f16 = cr_atan2f16(y, x),
    atan2pif16 = cr_atan2pif16(y, x),
    atanhf16 = cr_atanhf16(x),
    atanpif16 = cr_atanpif16(x),
    cbrtf16 = cr_cbrtf16(x),
    compoundf16 = cr_compoundf16(x, y),
    cosf16 = cr_cosf16(x),
    coshf16 = cr_coshf16(x),
    cospif16 = cr_cospif16(x),
    erff16 = cr_erff16(x),
    erfcf16 = cr_erfcf16(x),
    expf16 = cr_expf16(x),
    exp10f16 = cr_exp10f16(x),
    exp10m1f16 = cr_exp10m1f16(x),
    exp2f16 = cr_exp2f16(x),
    exp2m1f16 = cr_exp2m1f16(x),
    expm1f16 = cr_expm1f16(x),
    hypotf16 = cr_hypotf16(x, y),
    lgammaf16 = cr_lgammaf16(x),
    logf16 = cr_logf16(x),
    log10f16 = cr_log10f16(x),
    log10p1f16 = cr_log10p1f16(x),
    log1pf16 = cr_log1pf16(x),
    log2f16 = cr_log2f16(x),
    log2p1f16 = cr_log2p1f16(x),
    powf16 = cr_powf16(x, y),
    rsqrtf16 = cr_rsqrtf16(x),
    sinf16 = cr_sinf16(x),
    sinhf16 = cr_sinhf16(x),
    sinpif16 = cr_sinpif16(x),
    sqrtf16 = cr_sqrtf16(x),
    tanf16 = cr_tanf16(x),
    tanhf16 = cr_tanhf16(x),
    tanpif16 = cr_tanpif16(x),
    tgammaf16 = cr_tgammaf16(x),
});

#[cfg(feature = "f16")]
wrap_sincos!(f16, sincosf16 = cr_sincosf16);

/////// binary128 (`f128`) functions ///////

#[cfg(feature = "f128")]
wrap!(f128 {
    acosq = cr_acosq(x),
    asinq = cr_asinq(x),
    atanq = cr_atanq(x),
    atan2q = cr_atan2q(y, x),
    cbrtq = cr_cbrtq(x),
    expq = cr_expq(x),
    exp10q = cr_exp10q(x),
    exp2q = cr_exp2q(x),
    expm1q = cr_expm1q(x),
    hypotq = cr_hypotq(x, y),
    logq = cr_logq(x),
    rsqrtq = cr_rsqrtq(x),
    sqrtq = cr_sqrtq(x),
});
