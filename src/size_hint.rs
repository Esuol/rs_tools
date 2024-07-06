use std::cmp;

pub type SizeHint = (usize, Option<usize>);

#[inline]
pub fn add(a: SizeHint, b: SizeHint) -> SizeHint {
  let min = a.0.saturating_add(b.0);
}