#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]

#[test]
fn sanity() {
    assert!(core_math::expf(0.0).eq(&1.0));
}

#[test]
fn sincos_0() {
    let (s, c) = core_math::sincosf(-0.0);
    assert!(s.to_bits() == (-0_f32).to_bits());
    assert!(c.eq(&1.0));
}

#[test]
fn sincos_1() {
    let (s, c) = core_math::sincosf(1.0);
    assert!(core_math::sinf(1.0).eq(&s));
    assert!(core_math::cosf(1.0).eq(&c));
}

#[test]
fn compound() {
    assert!(core_math::compoundf(0.5, 1.0).eq(&1.5));
    assert!(core_math::compoundf(2.0, 0.0).eq(&1.0));
}

#[test]
fn gamma() {
    assert!(core_math::lgamma(1.0).eq(&0.0));
    assert!(core_math::tgamma(1.0).eq(&1.0));
}

#[test]
fn sincos64_0() {
    let (s, c) = core_math::sincos(-0.0);
    assert!(s.to_bits() == (-0_f64).to_bits());
    assert!(c.eq(&1.0));
}

#[test]
fn sincos64_1() {
    let (s, c) = core_math::sincos(1.0);
    assert!(core_math::sin(1.0).eq(&s));
    assert!(core_math::cos(1.0).eq(&c));
}

#[cfg(feature = "f16")]
#[test]
fn sanity_f16() {
    assert!(core_math::expf16(0.0).eq(&1.0));
    assert!(core_math::sqrtf16(4.0).eq(&2.0));

    let (s, c) = core_math::sincosf16(-0.0);
    assert!(s.to_bits() == (-0_f16).to_bits());
    assert!(c.eq(&1.0));
}

#[cfg(feature = "f128")]
#[test]
fn sanity_f128() {
    assert!(core_math::expq(0.0).eq(&1.0));
    assert!(core_math::sqrtq(4.0).eq(&2.0));
}
