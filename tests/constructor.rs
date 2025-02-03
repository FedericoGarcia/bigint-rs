#[cfg(test)]
mod bigint {
    use bigint;

    #[test]
    fn should_create_bigint_from_an_empty_string() {
        // When
        let bigint_value = bigint::BigInt::from_string("".to_string(), bigint::Base::Decimal);

        // Then
        assert_eq!(bigint_value.to_string(bigint::Base::Binary), "0");
        assert_eq!(bigint_value.to_string(bigint::Base::Octal), "0");
        assert_eq!(bigint_value.to_string(bigint::Base::Decimal), "0");
        assert_eq!(bigint_value.to_string(bigint::Base::Hexadecimal), "0");
    }

    #[test]
    fn should_create_bigint_from_a_string_with_zero() {
        // When
        let bigint_value = bigint::BigInt::from_string("0".to_string(), bigint::Base::Decimal);

        // Then
        assert_eq!(bigint_value.to_string(bigint::Base::Binary), "0");
        assert_eq!(bigint_value.to_string(bigint::Base::Octal), "0");
        assert_eq!(bigint_value.to_string(bigint::Base::Decimal), "0");
        assert_eq!(bigint_value.to_string(bigint::Base::Hexadecimal), "0");
    }

    #[test]
    fn should_create_bigint_from_a_string_with_a_single_digit() {
        // When
        let bigint_value = bigint::BigInt::from_string("1".to_string(), bigint::Base::Decimal);

        // Then
        assert_eq!(bigint_value.to_string(bigint::Base::Binary), "1");
        assert_eq!(bigint_value.to_string(bigint::Base::Octal), "1");
        assert_eq!(bigint_value.to_string(bigint::Base::Decimal), "1");
        assert_eq!(bigint_value.to_string(bigint::Base::Hexadecimal), "1");
    }

    #[test]
    fn should_create_bigint_from_a_string_with_a_small_number() {
        // When
        let bigint_value = bigint::BigInt::from_string("123".to_string(), bigint::Base::Decimal);

        // Then
        assert_eq!(bigint_value.to_string(bigint::Base::Binary), "1111011");
        assert_eq!(bigint_value.to_string(bigint::Base::Octal), "173");
        assert_eq!(bigint_value.to_string(bigint::Base::Decimal), "123");
        assert_eq!(bigint_value.to_string(bigint::Base::Hexadecimal), "7B");
    }

    #[test]
    fn should_create_bigint_from_a_string_with_a_big_number() {
        // When
        let bigint_value =
            bigint::BigInt::from_string("1234567890".to_string(), bigint::Base::Decimal);

        // Then
        assert_eq!(
            bigint_value.to_string(bigint::Base::Binary),
            "1001001100101100000001011010010"
        );
        assert_eq!(bigint_value.to_string(bigint::Base::Octal), "11145401322");
        assert_eq!(bigint_value.to_string(bigint::Base::Decimal), "1234567890");
        assert_eq!(
            bigint_value.to_string(bigint::Base::Hexadecimal),
            "499602D2"
        );
    }
}
