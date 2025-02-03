pub enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl Base {
    /// Returns the radix of the base.
    fn to_radix(&self) -> u8 {
        match self {
            Base::Binary => 2,
            Base::Octal => 8,
            Base::Decimal => 10,
            Base::Hexadecimal => 16,
        }
    }
}

/// A big integer data structure.
pub struct BigInt {
    /// The underlying data structure used to store the big integer.
    /// The data is stored in little-endian order.
    data: Vec<u8>,
}

impl BigInt {
    /// Create a new `BigInt` from a byte array.
    pub fn from_bytes(data: Vec<u8>) -> Self {
        BigInt { data }
    }

    /// Create a new `BigInt` from a string, using the provided base.
    pub fn from_string(data: String, base: Base) -> Self {
        let radix = base.to_radix();
        let mut result = BigInt::from_bytes(vec![]);

        for character in data.chars() {
            let value = match character {
                '0'..='9' => character as u8 - b'0',
                'A'..='Z' => character as u8 - b'A' + 10,
                'a'..='z' => character as u8 - b'a' + 10,
                _ => continue,
            };

            if value >= radix {
                panic!("Invalid character {} for base {}", character, radix);
            }

            result = result.multiply_by_small(radix) + BigInt::from_bytes(vec![value]);
        }

        result
    }

    /// Returns the number encoded as a string in the provided base.
    pub fn to_string(&self, base: Base) -> String {
        if self.is_zero() {
            return "0".to_string();
        }

        let radix = base.to_radix();
        let mut temp = self.clone();
        let mut digits = Vec::new();

        while !temp.is_zero() {
            let (quotient, remainder) = temp.divide_by_small(radix);

            digits.push(remainder);

            temp = quotient;
        }

        digits.reverse(); // The digits were obtained in reverse order.

        digits
            .into_iter()
            .map(|digit| {
                if digit < 10 {
                    (b'0' + digit) as char
                } else {
                    (b'A' + (digit - 10)) as char
                }
            })
            .collect()
    }

    /// Returns `true` if BigInt is zero.
    fn is_zero(&self) -> bool {
        if self.data.is_empty() {
            return true;
        }

        if self.data.iter().all(|&x| x == 0) {
            return true;
        }

        return false;
    }

    /// Multiply BigInt by a small number (u8).
    fn multiply_by_small(&self, multiplier: u8) -> BigInt {
        if self.is_zero() {
            return BigInt::from_bytes(vec![]);
        }

        let mut result = Vec::with_capacity(self.data.len() + 1);
        let mut carry: u16 = 0;

        for &digit in &self.data {
            let prod = digit as u16 * multiplier as u16 + carry;
            result.push((prod % 256) as u8);
            carry = prod / 256;
        }

        while carry > 0 {
            result.push((carry % 256) as u8);
            carry /= 256;
        }

        BigInt::from_bytes(result)
    }

    /// Divide BigInt by a small divisor (u8) and return the pair (quotient, remainder).
    fn divide_by_small(&self, divisor: u8) -> (BigInt, u8) {
        if self.is_zero() {
            return (BigInt::from_bytes(vec![]), 0);
        }

        let mut remainder: u16 = 0;
        let mut quotient = vec![0u8; self.data.len()];

        // Since the representation is little-endian, we iterate from the most significant byte.
        for i in (0..self.data.len()).rev() {
            remainder = remainder * 256 + self.data[i] as u16;
            quotient[i] = (remainder / divisor as u16) as u8;
            remainder %= divisor as u16;
        }

        // Remove leading zeros (in the little-endian representation, they are the last elements).
        while quotient.len() > 1 && *quotient.last().unwrap() == 0 {
            quotient.pop();
        }

        (BigInt::from_bytes(quotient), remainder as u8)
    }
}

impl std::clone::Clone for BigInt {
    fn clone(&self) -> Self {
        BigInt {
            data: self.data.clone(),
        }
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

        let mut result = Vec::with_capacity(longest_number_length);
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
            let bigint = BigInt::from_bytes(data.clone());

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
            assert_eq!(bigint.data, vec![0xD2, 0x02, 0x96, 0x49]);
        }

        #[test]
        fn should_create_a_bigint_from_a_string_hexadecimal() {
            // Given
            let data = "1A2B".to_string();

            // When
            let bigint = BigInt::from_string(data, Base::Hexadecimal);

            // Then
            assert_eq!(bigint.data, vec![0x2B, 0x1A]);
        }
    }
}

#[cfg(test)]
mod display {
    use super::*;

    #[cfg(test)]
    mod to_string {
        use super::*;

        #[test]
        fn should_display_bigint_with_0_bytes_in_base_10() {
            // Given
            let bigint = BigInt::from_bytes(vec![]);

            // When
            let result = format!("{}", bigint);

            // Then
            assert_eq!(result, "0");
        }

        #[test]
        fn should_display_bigint_with_1_byte_in_base_10() {
            // Given
            let bigint = BigInt::from_bytes(vec![0xE4]);

            // When
            let result = format!("{}", bigint);

            // Then
            assert_eq!(result, "228");
        }

        #[test]
        fn should_display_bigint_with_2_bytes_in_base_10() {
            // Given
            let bigint = BigInt::from_bytes(vec![0xFF, 0xFF]);

            // When
            let result = format!("{}", bigint);

            // Then
            assert_eq!(result, "65535");
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

#[cfg(test)]
mod utils {
    use super::*;

    // test is_zero
    mod is_zero {
        use super::*;

        #[test]
        fn should_return_true_when_bigint_is_zero() {
            // Given
            let bigint = BigInt::from_bytes(vec![]);

            // When
            let result = bigint.is_zero();

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn should_return_true_when_bigint_is_zero_with_data() {
            // Given
            let bigint = BigInt::from_bytes(vec![0, 0, 0]);

            // When
            let result = bigint.is_zero();

            // Then
            assert_eq!(result, true);
        }

        #[test]
        fn should_return_false_when_bigint_is_not_zero() {
            // Given
            let bigint = BigInt::from_bytes(vec![0, 0, 1]);

            // When
            let result = bigint.is_zero();

            // Then
            assert_eq!(result, false);
        }
    }

    mod multiply_by_small {
        use super::*;

        #[test]
        fn should_multiply_bigint_by_10() {
            // Given
            let bigint = BigInt::from_bytes(vec![0xD2, 0x04]);
            let expected = BigInt::from_bytes(vec![0x34, 0x30]);

            // When
            let result = bigint.multiply_by_small(10);

            // Then
            assert_eq!(result, expected);
        }
    }

    mod divide_by_small {
        use super::*;

        #[test]
        fn should_divide_bigint_by_10() {
            // Given
            let bigint = BigInt::from_bytes(vec![0x39, 0x30]);
            let expected = BigInt::from_bytes(vec![0xD2, 0x04]);

            // When
            let (quotient, _) = bigint.divide_by_small(10);

            // Then
            assert_eq!(quotient, expected);
        }

        #[test]
        fn should_divide_bigint_by_10_with_remainder() {
            // Given
            let bigint = BigInt::from_bytes(vec![0x39, 0x30]);
            let expected = 5;

            // When
            let (_, remainder) = bigint.divide_by_small(10);

            // Then
            assert_eq!(remainder, expected);
        }
    }
}
