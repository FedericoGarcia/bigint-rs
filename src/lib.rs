enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

/// A big integer data structure.
struct BigInt {
    /// The underlying data structure used to store the big integer.
    /// The data is stored in little-endian order.
    data: Vec<u8>,
}

impl BigInt {
    /// Create a new `BigInt` from a byte array.
    fn from_bytes(data: Vec<u8>) -> Self {
        BigInt { data }
    }

    /// Create a new `BigInt` from a string, using the provided base.
    fn from_string(data: String, base: Base) -> Self {
        let data = data.as_bytes();
        let mut result = vec![];

        for byte in data {
            let value = match byte {
                b'0'..=b'9' => byte - b'0',
                b'A'..=b'Z' => byte - b'A' + 10,
                b'a'..=b'z' => byte - b'a' + 10,
                _ => 0,
            };

            result.push(value);
        }

        Self::from_bytes(result)
    }

    /// Returns the number encoded as a string in the provided base.
    fn to_string(&self, base: Base) -> String {
        todo!()
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

impl std::fmt::Display for BigInt {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.to_string(Base::Decimal).fmt(formatter)
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

        let longest_number_length = left_data.len().max(right_data.len());

        let mut result = Vec::with_capacity(longest_number_length); // TODO: Check if should add 1 to capacity, is it more efficient?
        let mut carry = 0u8;

        for index in 0..longest_number_length {
            let left = *left_data.get(index).unwrap_or(&0) as u16;
            let right = *right_data.get(index).unwrap_or(&0) as u16;

            let total = left + right + carry as u16;

            result.push((total % 256) as u8);
            carry = (total / 256) as u8;
        }

        if carry > 0 {
            result.push(carry);
        }

        BigInt::from_bytes(result)
    }
}

#[cfg(test)]
mod constructor {
    use super::*;

    #[cfg(test)]
    mod from_bytes {
        use super::*;

        #[test]
        fn should_create_a_bigint_from_2_bytes() {
            // Given
            let data = vec![0xE4, 0x08];

            // When
            let bigint = BigInt::from_bytes(data);

            // Then
            assert_eq!(bigint.data[0], 0xE4);
            assert_eq!(bigint.data[1], 0x08);
        }
    }

    #[cfg(test)]
    mod from_string {
        use super::*;

        #[test]
        fn should_create_a_bigint_from_a_string() {
            // Given
            let data = "1234567890".to_string();

            // When
            let bigint = BigInt::from_string(data, Base::Decimal);

            // Then
            assert_eq!(bigint.data[0], 1);
            assert_eq!(bigint.data[1], 2);
            assert_eq!(bigint.data[2], 3);
            assert_eq!(bigint.data[3], 4);
            assert_eq!(bigint.data[4], 5);
            assert_eq!(bigint.data[5], 6);
            assert_eq!(bigint.data[6], 7);
            assert_eq!(bigint.data[7], 8);
            assert_eq!(bigint.data[8], 9);
            assert_eq!(bigint.data[9], 0);
        }
    }
}

#[cfg(test)]
mod display {
    use super::*;

    #[cfg(test)]
    mod to_string {
        use super::*;

        #[ignore]
        #[test]
        fn should_display_bigint_with_0_bytes_in_base_10() {
            // Given
            let bigint = BigInt::from_bytes(vec![]);

            // When
            let result = format!("{}", bigint);

            // Then
            assert_eq!(result, "0");
        }

        #[ignore]
        #[test]
        fn should_display_bigint_with_1_byte_in_base_10() {
            // Given
            let bigint = BigInt::from_bytes(vec![0xE4]);

            // When
            let result = format!("{}", bigint);

            // Then
            assert_eq!(result, "228");
        }

        #[ignore]
        #[test]
        fn should_display_bigint_with_2_bytes_in_base_10() {
            // Given
            let bigint = BigInt::from_bytes(vec![0xFF, 0xFF]);

            // When
            let result = format!("{}", bigint);

            // Then
            assert_eq!(result, "65536");
        }
    }
}

#[cfg(test)]
mod comparators {
    use super::*;

    #[cfg(test)]
    mod equal {
        use super::*;

        #[test]
        fn should_be_equal() {
            // Given
            let a = BigInt::from_bytes(vec![0xE4, 0x08]);
            let b = BigInt::from_bytes(vec![0xE4, 0x08]);

            // When
            let result = a == b;

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn should_not_be_equal() {
            // Given
            let a = BigInt::from_bytes(vec![0xE4, 0x08]);
            let b = BigInt::from_bytes(vec![0xE4]);

            // When
            let result = a == b;

            // Then
            assert_eq!(result, false);
        }
    }
}

#[cfg(test)]
mod operators {
    use super::*;

    #[cfg(test)]
    mod add {
        use super::*;

        #[test]
        fn should_add_2_bigint_with_same_data_length() {
            // Given
            let a = BigInt::from_bytes(vec![0xE4, 0x08]);
            let b = BigInt::from_bytes(vec![0xF1, 0x03]);

            let expected = BigInt::from_bytes(vec![0xD5, 0x0C]);

            // When
            let result = a + b;

            // Then
            assert_eq!(result, expected);
        }

        #[test]
        fn should_add_2_bigint_with_different_data_length() {
            // Given
            let a = BigInt::from_bytes(vec![0xE4, 0x08]);
            let b = BigInt::from_bytes(vec![0xF1, 0x03, 0x02]);

            let expected = BigInt::from_bytes(vec![0xD5, 0x0C, 0x02]);

            // When
            let result = a + b;

            // Then
            assert_eq!(result, expected);
        }

        #[test]
        fn should_add_2_bigint_with_different_data_length_and_overflow() {
            // Given
            let a = BigInt::from_bytes(vec![0xFF, 0xFF]);
            let b = BigInt::from_bytes(vec![0x01]);

            let expected = BigInt::from_bytes(vec![0x00, 0x00, 0x01]);

            // When
            let result = a + b;

            // Then
            assert_eq!(result, expected);
        }

        #[test]
        fn should_add_2_bigint_with_same_data_length_and_overflow() {
            // Given
            let a = BigInt::from_bytes(vec![0xFF, 0xFF]);
            let b = BigInt::from_bytes(vec![0x01, 0x01]);

            let expected = BigInt::from_bytes(vec![0x00, 0x01, 0x01]);

            // When
            let result = a + b;

            // Then
            assert_eq!(result, expected);
        }
    }
}
