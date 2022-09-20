pub struct Reader<T> {
    vector: Vec<T>,
    i: usize
}

impl<T> Reader<T> {
    pub fn new<I>(values: I) -> Self where I: IntoIterator<Item = T>{
        let iter_as_vector = values.into_iter().collect();
        Self {
            vector: iter_as_vector,
            i: 0
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        self.vector.get(i)
    }

    pub fn peek(&self, offset: usize) -> Option<&T> {
        self.vector.get(self.i + offset)
    }

    pub fn next(&mut self) {
        self.i += 1;
    }

    /// Returns a slice starting at the current index, inclusive, and continues as
    /// long as the elements match the provided predicate function. 
    pub fn take_while_inclusive<F>(&mut self, pred: F) -> &[T] where F: Fn(&T) -> bool {
        let starting_i = self.i;
        while let Some(val) = self.vector.get(self.i) {
            if pred(val) {
                self.i += 1;
            } else {
                break
            }
        }
        &self.vector[starting_i..self.i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(vec![], 0, None)]
    #[case(vec![], 1, None)]
    #[case(vec![0, 1, 2], 0, Some(0))]
    #[case(vec![0, 1, 2], 2, Some(2))]
    fn test_peek(#[case] iterable: Vec<i32>, #[case] ahead: usize, #[case] expected: Option<i32>) {
        let reader = Reader::new(iterable);
        match reader.peek(ahead) {
            Some(x) => assert_eq!(x, &expected.unwrap()),
            None => assert_eq!(None, expected)
        }
    }

    #[rstest]
    #[case(vec![], vec![], 0)]
    #[case(vec![1], vec![], 0)]
    #[case(vec![0, 2, 4, 3], vec![0, 2, 4], 3)]
    pub fn test_take_while_inclusive(#[case] items: Vec<i32>, #[case] expected_items: Vec<i32>, #[case] expected_i: usize) {
        let f = |x: &i32| x % 2 == 0;
        let mut reader = Reader::new(items);
        let actual_items = reader.take_while_inclusive(f);
        assert_eq!(actual_items, expected_items);
        assert_eq!(reader.i, expected_i);
    }
}