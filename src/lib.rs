type BigIntChunk = u8;

struct BigInt {
    data: Vec<BigIntChunk>,
}

impl BigInt {
    fn new(data: Vec<BigIntChunk>) -> Self {
        BigInt { data }
    }
}

impl std::fmt::Debug for BigInt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "BigInt ({}): [", self.data.len())?;

        for (index, value) in self.data.iter().enumerate() {
            write!(formatter, "{}", value)?;

            if index < self.data.len() - 1 {
                write!(formatter, ", ")?;
            }
        }

        write!(formatter, "]")?;

        Ok(())
    }
}

impl std::cmp::PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl std::ops::Add for BigInt {
    type Output = Self;

    fn add(self, right: Self) -> Self {
        let left_data = &self.data;
        let right_data = &right.data;

        let min_length = left_data.len().max(right_data.len());

        let mut result = vec![];
        let mut previous_overflow = false;

        for index in 0..min_length {
            let left = left_data.get(index).unwrap_or(&0);
            let right = right_data.get(index).unwrap_or(&0);

            let (sum, overflow) = left.overflowing_add(*right);

            let final_sum = if previous_overflow {
                let (sum, overflow) = sum.overflowing_add(1);
                previous_overflow = overflow;
                sum
            } else {
                previous_overflow = overflow;
                sum
            };

            result.push(final_sum);
        }

        if previous_overflow {
            result.push(previous_overflow as u8);
        }

        BigInt::new(result)
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

    #[test]
    fn should_add_2_bigint_with_same_data_length() {
        // Given
        let a = BigInt::new(vec![0xE4, 0x08]);
        let b = BigInt::new(vec![0xF1, 0x03]);

        let expected = BigInt::new(vec![0xD5, 0x0C]);

        // When
        let result = a + b;

        // Then
        assert_eq!(result, expected);
    }

    #[test]
    fn should_add_2_bigint_with_different_data_length() {
        // Given
        let a = BigInt::new(vec![0xE4, 0x08]);
        let b = BigInt::new(vec![0xF1, 0x03, 0x02]);

        let expected = BigInt::new(vec![0xD5, 0x0C, 0x02]);

        // When
        let result = a + b;

        // Then
        assert_eq!(result, expected);
    }
}
