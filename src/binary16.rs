//! Wrappers over CORE-MATH binary16 (`f16`) functions

use super::sys;

#[must_use]
#[inline]
pub fn acosf16(x: f16) -> f16 {
    unsafe { sys::cr_acosf16(x) }
}

#[must_use]
#[inline]
pub fn acoshf16(x: f16) -> f16 {
    unsafe { sys::cr_acoshf16(x) }
}

#[must_use]
#[inline]
pub fn acospif16(x: f16) -> f16 {
    unsafe { sys::cr_acospif16(x) }
}

#[must_use]
#[inline]
pub fn asinf16(x: f16) -> f16 {
    unsafe { sys::cr_asinf16(x) }
}

#[must_use]
#[inline]
pub fn asinhf16(x: f16) -> f16 {
    unsafe { sys::cr_asinhf16(x) }
}

#[must_use]
#[inline]
pub fn asinpif16(x: f16) -> f16 {
    unsafe { sys::cr_asinpif16(x) }
}

#[must_use]
#[inline]
pub fn atanf16(x: f16) -> f16 {
    unsafe { sys::cr_atanf16(x) }
}

#[must_use]
#[inline]
pub fn atan2f16(y: f16, x: f16) -> f16 {
    unsafe { sys::cr_atan2f16(y, x) }
}

#[must_use]
#[inline]
pub fn atan2pif16(y: f16, x: f16) -> f16 {
    unsafe { sys::cr_atan2pif16(y, x) }
}

#[must_use]
#[inline]
pub fn atanhf16(x: f16) -> f16 {
    unsafe { sys::cr_atanhf16(x) }
}

#[must_use]
#[inline]
pub fn atanpif16(x: f16) -> f16 {
    unsafe { sys::cr_atanpif16(x) }
}

#[must_use]
#[inline]
pub fn cbrtf16(x: f16) -> f16 {
    unsafe { sys::cr_cbrtf16(x) }
}

#[must_use]
#[inline]
pub fn compoundf16(x: f16, y: f16) -> f16 {
    unsafe { sys::cr_compoundf16(x, y) }
}

#[must_use]
#[inline]
pub fn cosf16(x: f16) -> f16 {
    unsafe { sys::cr_cosf16(x) }
}

#[must_use]
#[inline]
pub fn coshf16(x: f16) -> f16 {
    unsafe { sys::cr_coshf16(x) }
}

#[must_use]
#[inline]
pub fn cospif16(x: f16) -> f16 {
    unsafe { sys::cr_cospif16(x) }
}

#[must_use]
#[inline]
pub fn erff16(x: f16) -> f16 {
    unsafe { sys::cr_erff16(x) }
}

#[must_use]
#[inline]
pub fn erfcf16(x: f16) -> f16 {
    unsafe { sys::cr_erfcf16(x) }
}

#[must_use]
#[inline]
pub fn expf16(x: f16) -> f16 {
    unsafe { sys::cr_expf16(x) }
}

#[must_use]
#[inline]
pub fn exp10f16(x: f16) -> f16 {
    unsafe { sys::cr_exp10f16(x) }
}

#[must_use]
#[inline]
pub fn exp10m1f16(x: f16) -> f16 {
    unsafe { sys::cr_exp10m1f16(x) }
}

#[must_use]
#[inline]
pub fn exp2f16(x: f16) -> f16 {
    unsafe { sys::cr_exp2f16(x) }
}

#[must_use]
#[inline]
pub fn exp2m1f16(x: f16) -> f16 {
    unsafe { sys::cr_exp2m1f16(x) }
}

#[must_use]
#[inline]
pub fn expm1f16(x: f16) -> f16 {
    unsafe { sys::cr_expm1f16(x) }
}

#[must_use]
#[inline]
pub fn hypotf16(x: f16, y: f16) -> f16 {
    unsafe { sys::cr_hypotf16(x, y) }
}

#[must_use]
#[inline]
pub fn lgammaf16(x: f16) -> f16 {
    unsafe { sys::cr_lgammaf16(x) }
}

#[must_use]
#[inline]
pub fn logf16(x: f16) -> f16 {
    unsafe { sys::cr_logf16(x) }
}

#[must_use]
#[inline]
pub fn log10f16(x: f16) -> f16 {
    unsafe { sys::cr_log10f16(x) }
}

#[must_use]
#[inline]
pub fn log10p1f16(x: f16) -> f16 {
    unsafe { sys::cr_log10p1f16(x) }
}

#[must_use]
#[inline]
pub fn log1pf16(x: f16) -> f16 {
    unsafe { sys::cr_log1pf16(x) }
}

#[must_use]
#[inline]
pub fn log2f16(x: f16) -> f16 {
    unsafe { sys::cr_log2f16(x) }
}

#[must_use]
#[inline]
pub fn log2p1f16(x: f16) -> f16 {
    unsafe { sys::cr_log2p1f16(x) }
}

#[must_use]
#[inline]
pub fn powf16(x: f16, y: f16) -> f16 {
    unsafe { sys::cr_powf16(x, y) }
}

#[must_use]
#[inline]
pub fn rsqrtf16(x: f16) -> f16 {
    unsafe { sys::cr_rsqrtf16(x) }
}

#[must_use]
#[inline]
pub fn sinf16(x: f16) -> f16 {
    unsafe { sys::cr_sinf16(x) }
}

#[must_use]
#[inline]
pub fn sincosf16(x: f16) -> (f16, f16) {
    use core::mem::MaybeUninit;
    let mut s = MaybeUninit::uninit();
    let mut c = MaybeUninit::uninit();

    unsafe {
        sys::cr_sincosf16(x, s.as_mut_ptr(), c.as_mut_ptr());
        (s.assume_init(), c.assume_init())
    }
}

#[must_use]
#[inline]
pub fn sinhf16(x: f16) -> f16 {
    unsafe { sys::cr_sinhf16(x) }
}

#[must_use]
#[inline]
pub fn sinpif16(x: f16) -> f16 {
    unsafe { sys::cr_sinpif16(x) }
}

#[must_use]
#[inline]
pub fn sqrtf16(x: f16) -> f16 {
    unsafe { sys::cr_sqrtf16(x) }
}

#[must_use]
#[inline]
pub fn tanf16(x: f16) -> f16 {
    unsafe { sys::cr_tanf16(x) }
}

#[must_use]
#[inline]
pub fn tanhf16(x: f16) -> f16 {
    unsafe { sys::cr_tanhf16(x) }
}

#[must_use]
#[inline]
pub fn tanpif16(x: f16) -> f16 {
    unsafe { sys::cr_tanpif16(x) }
}

#[must_use]
#[inline]
pub fn tgammaf16(x: f16) -> f16 {
    unsafe { sys::cr_tgammaf16(x) }
}
