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
}
