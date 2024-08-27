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