pub fn add(a: usize, b: usize) -> usize {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math() {
        assert_eq!(add(1, 1), 2);
    }
}
