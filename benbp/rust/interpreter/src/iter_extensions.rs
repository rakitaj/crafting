use std::iter::Peekable;

pub struct TakeUntil<'a, T: Iterator + 'a, P: FnMut(&T::Item) -> bool>
where
    T::Item: 'a,
{
    inner: &'a mut Peekable<T>,
    condition: P,
}

impl<'a, T: Iterator, P> Iterator for TakeUntil<'a, T, P>
where
    P: FnMut(&T::Item) -> bool,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<T::Item> {
        let return_next = match self.inner.peek() {
            Some(ref v) => (self.condition)(v),
            _ => false,
        };
        if return_next {
            self.inner.next()
        } else {
            None
        }
    }
}

pub trait TakeUntilable<'a, T: Iterator>: Iterator {
    fn take_until<P: FnMut(&T::Item) -> bool>(&'a mut self, f: P) -> TakeUntil<'a, T, P>;
}

impl<'a, T: Iterator> TakeUntilable<'a, T> for Peekable<T> {
    fn take_until<P: FnMut(&T::Item) -> bool>(&'a mut self, f: P) -> TakeUntil<'a, T, P> {
        TakeUntil {
            inner: self,
            condition: f,
        }
    }
}
