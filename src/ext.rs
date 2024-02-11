use std::mem;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Unfold<T, F> {
    value: Option<T>,
    f: F,
}

impl<T, F: Fn(&T) -> Option<T>> Iterator for Unfold<T, F> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let new_value = (self.f)(self.value.as_ref()?);
        mem::replace(&mut self.value, new_value)
    }
}

pub trait Unfolding<T> {
    fn unfold<F: Fn(&T) -> Option<T>>(self, f: F) -> Unfold<T, F>;
}

impl<T> Unfolding<T> for Option<T> {
    fn unfold<F: Fn(&T) -> Option<T>>(self, f: F) -> Unfold<T, F> {
        Unfold { value: self, f }
    }
}
