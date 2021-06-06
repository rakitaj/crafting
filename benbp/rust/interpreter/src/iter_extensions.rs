use std::iter::Peekable;

pub struct TakeWhileExclusive<'a, T: Iterator + 'a, P: FnMut(&T::Item) -> bool>
where
    T::Item: 'a,
{
    inner: &'a mut Peekable<T>,
    condition: P,
}

impl<'a, T: Iterator, P> Iterator for TakeWhileExclusive<'a, T, P>
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

pub trait TakeWhileExclusiveable<'a, T: Iterator>: Iterator {
    fn take_while_exclusive<P: FnMut(&T::Item) -> bool>(&'a mut self, f: P) -> TakeWhileExclusive<'a, T, P>;
}

impl<'a, T: Iterator> TakeWhileExclusiveable<'a, T> for Peekable<T> {
    fn take_while_exclusive<P: FnMut(&T::Item) -> bool>(&'a mut self, f: P) -> TakeWhileExclusive<'a, T, P> {
        TakeWhileExclusive {
            inner: self,
            condition: f,
        }
    }
}
