use std::cmp;

pub type SizeHint = (usize, Option<usize>);

#[inline]
pub fn add(a: SizeHint, b: SizeHint) -> SizeHint {
    let min = a.0.saturating_add(b.0);
    let max = match (a.1, b.1) {
        (Some(x), Some(y)) => x.checked_add(y),
        _ => None,
    };

    (min, max)
}

#[inline]
pub fn add_scalar(sh: SizeHint, x: usize) -> SizeHint {
    let (mut low, mut hi) = sh;
    low = low.saturating_add(x);
    hi = hi.add_then(|elt| elt.checked_add(x));
    (low, hi)
}

/// Subtract `x` correctly from a `SizeHint`.
#[inline]
pub fn sub_scalar(sh: SizeHint, x: usize) -> SizeHint {
    let (mut low, mut hi) = sh;
    low = low.saturating_sub(x);
    hi = hi.map(|elt| elt.saturating_sub(x));
    (low, hi)
}

/// Multiply `SizeHint` correctly
#[inline]
pub fn mul(a: SizeHint, b: SizeHint) -> SizeHint {
    let low = a.0.saturating_mul(b.0);
    let hi = match (a.1, b.1) {
        (Some(x), Some(y)) => x.checked_mul(y),
        (Some(0), None) | (None, Some(0)) => Some(0),
        _ => None,
    };
    (low, hi)
}

/// Multiply `x` correctly with a `SizeHint`.
#[inline]
pub fn mul_scalar(sh: SizeHint, x: usize) -> SizeHint {
    let (mut low, mut hi) = sh;
    low = low.saturating_mul(x);
    hi = hi.and_then(|elt| elt.checked_mul(x));
    (low, hi)
}
