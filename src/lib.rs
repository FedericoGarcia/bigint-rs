struct BigInt {
    data: Vec<usize>,
}

impl BigInt {
    fn new(data: Vec<usize>) -> Self {
        BigInt { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_a_bigint_from_2_bytes() {
        let bigint = BigInt::new(vec![0xE4, 0x08]);

        assert_eq!(bigint.data[0], 0xE4);
        assert_eq!(bigint.data[1], 0x08);
    }
}
