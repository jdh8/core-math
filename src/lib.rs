#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

use core_math_sys as sys;

/////// `f32` functions ///////

#[must_use]
#[inline]
pub fn acosf(x: f32) -> f32 {
    unsafe { sys::cr_acosf(x) }
}

#[must_use]
#[inline]
pub fn acoshf(x: f32) -> f32 {
    unsafe { sys::cr_acoshf(x) }
}

#[must_use]
#[inline]
pub fn acospif(x: f32) -> f32 {
    unsafe { sys::cr_acospif(x) }
}

#[must_use]
#[inline]
pub fn asinf(x: f32) -> f32 {
    unsafe { sys::cr_asinf(x) }
}

#[must_use]
#[inline]
pub fn asinhf(x: f32) -> f32 {
    unsafe { sys::cr_asinhf(x) }
}

#[must_use]
#[inline]
pub fn asinpif(x: f32) -> f32 {
    unsafe { sys::cr_asinpif(x) }
}

#[must_use]
#[inline]
pub fn atanf(x: f32) -> f32 {
    unsafe { sys::cr_atanf(x) }
}

#[must_use]
#[inline]
pub fn atan2f(y: f32, x: f32) -> f32 {
    unsafe { sys::cr_atan2f(y, x) }
}

#[must_use]
#[inline]
pub fn atan2pif(y: f32, x: f32) -> f32 {
    unsafe { sys::cr_atan2pif(y, x) }
}

#[must_use]
#[inline]
pub fn atanhf(x: f32) -> f32 {
    unsafe { sys::cr_atanhf(x) }
}

#[must_use]
#[inline]
pub fn atanpif(x: f32) -> f32 {
    unsafe { sys::cr_atanpif(x) }
}

#[must_use]
#[inline]
pub fn cbrtf(x: f32) -> f32 {
    unsafe { sys::cr_cbrtf(x) }
}

#[must_use]
#[inline]
pub fn cosf(x: f32) -> f32 {
    unsafe { sys::cr_cosf(x) }
}

#[must_use]
#[inline]
pub fn coshf(x: f32) -> f32 {
    unsafe { sys::cr_coshf(x) }
}

#[must_use]
#[inline]
pub fn cospif(x: f32) -> f32 {
    unsafe { sys::cr_cospif(x) }
}

#[must_use]
#[inline]
pub fn erff(x: f32) -> f32 {
    unsafe { sys::cr_erff(x) }
}

#[must_use]
#[inline]
pub fn erfcf(x: f32) -> f32 {
    unsafe { sys::cr_erfcf(x) }
}

#[must_use]
#[inline]
pub fn expf(x: f32) -> f32 {
    unsafe { sys::cr_expf(x) }
}

#[must_use]
#[inline]
pub fn exp10f(x: f32) -> f32 {
    unsafe { sys::cr_exp10f(x) }
}

#[must_use]
#[inline]
pub fn exp10m1f(x: f32) -> f32 {
    unsafe { sys::cr_exp10m1f(x) }
}

#[must_use]
#[inline]
pub fn exp2f(x: f32) -> f32 {
    unsafe { sys::cr_exp2f(x) }
}

#[must_use]
#[inline]
pub fn exp2m1f(x: f32) -> f32 {
    unsafe { sys::cr_exp2m1f(x) }
}

#[must_use]
#[inline]
pub fn expm1f(x: f32) -> f32 {
    unsafe { sys::cr_expm1f(x) }
}

#[must_use]
#[inline]
pub fn hypotf(x: f32, y: f32) -> f32 {
    unsafe { sys::cr_hypotf(x, y) }
}

#[must_use]
#[inline]
pub fn lgammaf(x: f32) -> f32 {
    unsafe { sys::cr_lgammaf(x) }
}

#[must_use]
#[inline]
pub fn logf(x: f32) -> f32 {
    unsafe { sys::cr_logf(x) }
}

#[must_use]
#[inline]
pub fn log10f(x: f32) -> f32 {
    unsafe { sys::cr_log10f(x) }
}

#[must_use]
#[inline]
pub fn log10p1f(x: f32) -> f32 {
    unsafe { sys::cr_log10p1f(x) }
}

#[must_use]
#[inline]
pub fn log1pf(x: f32) -> f32 {
    unsafe { sys::cr_log1pf(x) }
}

#[must_use]
#[inline]
pub fn log2f(x: f32) -> f32 {
    unsafe { sys::cr_log2f(x) }
}

#[must_use]
#[inline]
pub fn log2p1f(x: f32) -> f32 {
    unsafe { sys::cr_log2p1f(x) }
}

#[must_use]
#[inline]
pub fn powf(x: f32, y: f32) -> f32 {
    unsafe { sys::cr_powf(x, y) }
}

#[must_use]
#[inline]
pub fn rsqrtf(x: f32) -> f32 {
    unsafe { sys::cr_rsqrtf(x) }
}

#[must_use]
#[inline]
pub fn sinf(x: f32) -> f32 {
    unsafe { sys::cr_sinf(x) }
}

#[must_use]
#[inline]
pub fn sincosf(x: f32) -> f32 {
    unsafe { sys::cr_sincosf(x) }
}

#[must_use]
#[inline]
pub fn sinhf(x: f32) -> f32 {
    unsafe { sys::cr_sinhf(x) }
}

#[must_use]
#[inline]
pub fn sinpif(x: f32) -> f32 {
    unsafe { sys::cr_sinpif(x) }
}

#[must_use]
#[inline]
pub fn tanf(x: f32) -> f32 {
    unsafe { sys::cr_tanf(x) }
}

#[must_use]
#[inline]
pub fn tanhf(x: f32) -> f32 {
    unsafe { sys::cr_tanhf(x) }
}

#[must_use]
#[inline]
pub fn tanpif(x: f32) -> f32 {
    unsafe { sys::cr_tanpif(x) }
}

#[must_use]
#[inline]
pub fn tgammaf(x: f32) -> f32 {
    unsafe { sys::cr_tgammaf(x) }
}

/////// `f64` functions ///////

#[must_use]
#[inline]
pub fn acos(x: f64) -> f64 {
    unsafe { sys::cr_acos(x) }
}

#[must_use]
#[inline]
pub fn acosh(x: f64) -> f64 {
    unsafe { sys::cr_acosh(x) }
}

#[must_use]
#[inline]
pub fn acospi(x: f64) -> f64 {
    unsafe { sys::cr_acospi(x) }
}

#[must_use]
#[inline]
pub fn asin(x: f64) -> f64 {
    unsafe { sys::cr_asin(x) }
}

#[must_use]
#[inline]
pub fn asinh(x: f64) -> f64 {
    unsafe { sys::cr_asinh(x) }
}

#[must_use]
#[inline]
pub fn asinpi(x: f64) -> f64 {
    unsafe { sys::cr_asinpi(x) }
}

#[must_use]
#[inline]
pub fn atan(x: f64) -> f64 {
    unsafe { sys::cr_atan(x) }
}

#[must_use]
#[inline]
pub fn atan2(y: f64, x: f64) -> f64 {
    unsafe { sys::cr_atan2(y, x) }
}

#[must_use]
#[inline]
pub fn atan2pi(y: f64, x: f64) -> f64 {
    unsafe { sys::cr_atan2pi(y, x) }
}

#[must_use]
#[inline]
pub fn atanh(x: f64) -> f64 {
    unsafe { sys::cr_atanh(x) }
}

#[must_use]
#[inline]
pub fn atanpi(x: f64) -> f64 {
    unsafe { sys::cr_atanpi(x) }
}

#[must_use]
#[inline]
pub fn cbrt(x: f64) -> f64 {
    unsafe { sys::cr_cbrt(x) }
}

#[must_use]
#[inline]
pub fn cos(x: f64) -> f64 {
    unsafe { sys::cr_cos(x) }
}

#[must_use]
#[inline]
pub fn cosh(x: f64) -> f64 {
    unsafe { sys::cr_cosh(x) }
}

#[must_use]
#[inline]
pub fn cospi(x: f64) -> f64 {
    unsafe { sys::cr_cospi(x) }
}

#[must_use]
#[inline]
pub fn erf(x: f64) -> f64 {
    unsafe { sys::cr_erf(x) }
}

#[must_use]
#[inline]
pub fn erfc(x: f64) -> f64 {
    unsafe { sys::cr_erfc(x) }
}

#[must_use]
#[inline]
pub fn exp(x: f64) -> f64 {
    unsafe { sys::cr_exp(x) }
}

#[must_use]
#[inline]
pub fn exp10(x: f64) -> f64 {
    unsafe { sys::cr_exp10(x) }
}

#[must_use]
#[inline]
pub fn exp10m1(x: f64) -> f64 {
    unsafe { sys::cr_exp10m1(x) }
}

#[must_use]
#[inline]
pub fn exp2(x: f64) -> f64 {
    unsafe { sys::cr_exp2(x) }
}

#[must_use]
#[inline]
pub fn exp2m1(x: f64) -> f64 {
    unsafe { sys::cr_exp2m1(x) }
}

#[must_use]
#[inline]
pub fn expm1(x: f64) -> f64 {
    unsafe { sys::cr_expm1(x) }
}

#[must_use]
#[inline]
pub fn hypot(x: f64, y: f64) -> f64 {
    unsafe { sys::cr_hypot(x, y) }
}

#[must_use]
#[inline]
pub fn log(x: f64) -> f64 {
    unsafe { sys::cr_log(x) }
}

#[must_use]
#[inline]
pub fn log10(x: f64) -> f64 {
    unsafe { sys::cr_log10(x) }
}

#[must_use]
#[inline]
pub fn log10p1(x: f64) -> f64 {
    unsafe { sys::cr_log10p1(x) }
}

#[must_use]
#[inline]
pub fn log1p(x: f64) -> f64 {
    unsafe { sys::cr_log1p(x) }
}

#[must_use]
#[inline]
pub fn log2(x: f64) -> f64 {
    unsafe { sys::cr_log2(x) }
}

#[must_use]
#[inline]
pub fn log2p1(x: f64) -> f64 {
    unsafe { sys::cr_log2p1(x) }
}

#[must_use]
#[inline]
pub fn pow(x: f64, y: f64) -> f64 {
    unsafe { sys::cr_pow(x, y) }
}

#[must_use]
#[inline]
pub fn rsqrt(x: f64) -> f64 {
    unsafe { sys::cr_rsqrt(x) }
}

#[must_use]
#[inline]
pub fn sin(x: f64) -> f64 {
    unsafe { sys::cr_sin(x) }
}

#[must_use]
#[inline]
pub fn sinh(x: f64) -> f64 {
    unsafe { sys::cr_sinh(x) }
}

#[must_use]
#[inline]
pub fn sinpi(x: f64) -> f64 {
    unsafe { sys::cr_sinpi(x) }
}

#[must_use]
#[inline]
pub fn tan(x: f64) -> f64 {
    unsafe { sys::cr_tan(x) }
}

#[must_use]
#[inline]
pub fn tanh(x: f64) -> f64 {
    unsafe { sys::cr_tanh(x) }
}

#[must_use]
#[inline]
pub fn tanpi(x: f64) -> f64 {
    unsafe { sys::cr_tanpi(x) }
}
