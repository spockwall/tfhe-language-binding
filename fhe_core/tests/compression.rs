#[cfg(test)]
mod compression_tests {

    use fhe_core::utils::rle_compression::{
        bytes_to_rle, rle_compress, rle_decompress, rle_to_bytes,
    };
    #[test]
    fn compression() {
        let test_cases = vec![
            vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0],
            vec![3, 1, 2, 3, 3, 3, 30, 0, 0, 0, 0, 0],
            vec![0, 0, 1],
            vec![1, 0, 1],
            vec![0, 0, 0, 1],
            vec![3],
        ];
        for test_case in test_cases {
            let compressed = rle_compress(&test_case);
            let decompressed = rle_decompress(&compressed);
            assert_eq!(test_case, decompressed);
        }
    }

    #[test]
    fn rle_type_conversion() {
        let test_cases = vec![
            vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0],
            vec![3, 1, 2, 3, 3, 3, 30, 0, 0, 0, 0, 0],
            vec![0, 0, 1],
            vec![1, 0, 1],
            vec![0, 0, 0, 1],
            vec![3],
        ];
        for test_case in test_cases {
            let compressed = rle_compress(&test_case);
            let bytes = rle_to_bytes(&compressed);
            let rle = bytes_to_rle(&bytes);
            let decompressed = rle_decompress(&rle);
            assert_eq!(test_case, decompressed);
        }
    }
}
