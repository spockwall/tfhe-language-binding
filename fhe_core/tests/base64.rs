#[cfg(test)]
mod tests {
    use fhe_core::utils::base64::{decode, encode};
    #[test]
    fn test_encode() {
        let input: &str = "Hello, World!";
        let expected: &str = "SGVsbG8sIFdvcmxkIQ==";
        let result: String = encode(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_decode() {
        let input: &str = "SGVsbG8sIFdvcmxkIQ==";
        let expected: &str = "Hello, World!";
        let result = decode(input);
        assert_eq!(result.unwrap(), expected);
    }
}
