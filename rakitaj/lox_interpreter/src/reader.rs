pub struct Reader<T> {
    vector: Vec<T>,
    i: usize
}

impl<T> Reader<T> {
    pub fn new(values: T) -> Self where T: IntoIterator<Item = T>{
        let iter_as_vector = values.into_iter().collect();
        Self {
            vector: iter_as_vector,
            i: 0
        }
    }

    pub fn peek(&self, n: usize) -> Option<&T> {
        self.vector.get(self.i + n)
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
        assert_eq!(reader.peek(n), expected);
    }
}