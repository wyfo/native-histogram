// Reimplementation of `libm::frexp` without special case (NaN, Inf, etc.) handling
pub fn frexp(x: f64) -> (f64, i32) {
    let mut y = x.to_bits();
    let ee = ((y >> 52) & 0x7ff) as i32;

    let e = ee - 0x3fe;
    y &= 0x800fffffffffffff;
    y |= 0x3fe0000000000000;
    (f64::from_bits(y), e)
}

// Reimplementation of `f64::next_down` without special case (NaN, Inf, etc.) handling
pub fn prev_float(value: f64) -> f64 {
    f64::from_bits(value.to_bits().saturating_sub(1))
}
