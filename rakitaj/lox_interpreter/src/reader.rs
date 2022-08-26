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

    pub fn get(&self, n: usize) -> Option<&T> {
        self.vector.get(n)
    }

    pub fn peek(&self, offset: usize) -> Option<&T> {
        self.vector.get(self.i + offset)
    }

    pub fn next(&mut self) {
        self.i += 1;
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
}