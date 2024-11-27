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
        let mut result: Self = BigInt::new(vec![]);

        let mut previous_carry: bool = false;

        for (index, value) in self.data.iter().enumerate() {
            let (sum1, carry1) = value.overflowing_add(previous_carry as BigIntChunk);
            let (sum2, carry2) = sum1.overflowing_add(right.data[index]);

            result.data.push(sum2);

            previous_carry = carry1 || carry2;
        }

        result
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
    fn should_add_2_bigint() {
        // Given
        let a = BigInt::new(vec![0xE4, 0x08]);
        let b = BigInt::new(vec![0xF1, 0x03]);

        let expected = BigInt::new(vec![0xD5, 0x0C]);

        // When
        let result = a + b;

        // Then
        assert_eq!(result, expected);
    }
}
