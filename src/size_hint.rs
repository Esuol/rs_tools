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
