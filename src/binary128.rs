//! Wrappers over CORE-MATH binary128 (`f128`) functions

use super::sys;

#[must_use]
#[inline]
pub fn acosq(x: f128) -> f128 {
    unsafe { sys::cr_acosq(x) }
}

#[must_use]
#[inline]
pub fn asinq(x: f128) -> f128 {
    unsafe { sys::cr_asinq(x) }
}

#[must_use]
#[inline]
pub fn atanq(x: f128) -> f128 {
    unsafe { sys::cr_atanq(x) }
}

#[must_use]
#[inline]
pub fn atan2q(y: f128, x: f128) -> f128 {
    unsafe { sys::cr_atan2q(y, x) }
}

#[must_use]
#[inline]
pub fn cbrtq(x: f128) -> f128 {
    unsafe { sys::cr_cbrtq(x) }
}

#[must_use]
#[inline]
pub fn expq(x: f128) -> f128 {
    unsafe { sys::cr_expq(x) }
}

#[must_use]
#[inline]
pub fn exp10q(x: f128) -> f128 {
    unsafe { sys::cr_exp10q(x) }
}

#[must_use]
#[inline]
pub fn exp2q(x: f128) -> f128 {
    unsafe { sys::cr_exp2q(x) }
}

#[must_use]
#[inline]
pub fn expm1q(x: f128) -> f128 {
    unsafe { sys::cr_expm1q(x) }
}

#[must_use]
#[inline]
pub fn hypotq(x: f128, y: f128) -> f128 {
    unsafe { sys::cr_hypotq(x, y) }
}

#[must_use]
#[inline]
pub fn logq(x: f128) -> f128 {
    unsafe { sys::cr_logq(x) }
}

#[must_use]
#[inline]
pub fn rsqrtq(x: f128) -> f128 {
    unsafe { sys::cr_rsqrtq(x) }
}

#[must_use]
#[inline]
pub fn sqrtq(x: f128) -> f128 {
    unsafe { sys::cr_sqrtq(x) }
}
