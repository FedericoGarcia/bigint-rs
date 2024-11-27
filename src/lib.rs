struct BigInt {
    data: Vec<usize>,
}

impl BigInt {
    fn new(data: Vec<usize>) -> Self {
        BigInt { data }
    }
}

impl std::cmp::PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_bigint_from_2_bytes() {
        // Given
        let data = vec![0xE4, 0x08];

        // When
        let bigint = BigInt::new(data);

        // Then
        assert_eq!(bigint.data[0], 0xE4);
        assert_eq!(bigint.data[1], 0x08);
    }

    #[test]
    fn should_be_equal() {
        // Given
        let a = BigInt::new(vec![0xE4, 0x08]);
        let b = BigInt::new(vec![0xE4, 0x08]);

        // When
        let result = a == b;

        // Then
        assert_eq!(result, true);
    }

    #[test]
    fn should_not_be_equal() {
        // Given
        let a = BigInt::new(vec![0xE4, 0x08]);
        let b = BigInt::new(vec![0xE4]);

        // When
        let result = a == b;

        // Then
        assert_eq!(result, false);
    }
}
