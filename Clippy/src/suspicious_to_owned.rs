use std::borrow::Cow;

pub fn take(c: &Cow<'_, str>) -> usize {
    let owned = c.to_owned();
    owned.len()
}
