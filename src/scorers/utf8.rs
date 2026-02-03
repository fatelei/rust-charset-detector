/// UTF-8 validation and scoring
///
/// Provides fast UTF-8 validation with confidence scoring
use crate::models::{DetectionMethod, DetectionResult, Encoding};
use crate::ranges::is_utf8_continuation;

/// Validate and score UTF-8 encoding
pub fn validate_utf8(data: &[u8]) -> Option<DetectionResult> {
    if data.is_empty() {
        return None;
    }

    // Check for BOM
    if data.len() >= 3 && &data[0..3] == b"\xEF\xBB\xBF" {
        // Still validate the rest
        let (valid, invalid_count) = count_valid_sequences(&data[3..]);
        let confidence = if invalid_count == 0 { 0.99 } else { 0.85 };
        return Some(
            DetectionResult::new(Encoding::Utf8, confidence).with_method(DetectionMethod::Bom),
        );
    }

    let (valid, invalid_count) = count_valid_sequences(data);

    if invalid_count == 0 {
        // Perfect UTF-8
        let confidence = if valid > 0 { 0.98 } else { 0.5 };
        Some(
            DetectionResult::new(Encoding::Utf8, confidence)
                .with_method(DetectionMethod::Utf8Validation),
        )
    } else if valid > invalid_count * 4 {
        // Mostly valid UTF-8
        let ratio = valid as f32 / (valid + invalid_count) as f32;
        let confidence = 0.6 * ratio;
        Some(
            DetectionResult::new(Encoding::Utf8, confidence)
                .with_method(DetectionMethod::Utf8Validation),
        )
    } else {
        // Probably not UTF-8
        None
    }
}

/// Count valid and invalid UTF-8 sequences
fn count_valid_sequences(data: &[u8]) -> (usize, usize) {
    let mut valid = 0;
    let mut invalid = 0;
    let mut i = 0;

    while i < data.len() {
        let byte = data[i];

        if byte < 0x80 {
            // ASCII (0-127)
            valid += 1;
            i += 1;
        } else if byte >= 0xC2 && byte <= 0xDF {
            // 2-byte sequence
            if i + 1 < data.len() && is_utf8_continuation(data[i + 1]) {
                valid += 1;
                i += 2;
            } else {
                invalid += 1;
                i += 1;
            }
        } else if byte >= 0xE0 && byte <= 0xEF {
            // 3-byte sequence
            if i + 2 < data.len()
                && is_utf8_continuation(data[i + 1])
                && is_utf8_continuation(data[i + 2])
            {
                // Check for overlong encoding
                if byte == 0xE0 && data[i + 1] < 0xA0 {
                    invalid += 1;
                } else if byte == 0xED && data[i + 1] >= 0xA0 {
                    // Surrogate range (invalid)
                    invalid += 1;
                } else {
                    valid += 1;
                }
                i += 3;
            } else {
                invalid += 1;
                i += 1;
            }
        } else if byte >= 0xF0 && byte <= 0xF4 {
            // 4-byte sequence
            if i + 3 < data.len()
                && is_utf8_continuation(data[i + 1])
                && is_utf8_continuation(data[i + 2])
                && is_utf8_continuation(data[i + 3])
            {
                // Check for overlong and out-of-range
                if byte == 0xF0 && data[i + 1] < 0x90 {
                    invalid += 1;
                } else if byte == 0xF4 && data[i + 1] > 0x8F {
                    invalid += 1;
                } else {
                    valid += 1;
                }
                i += 4;
            } else {
                invalid += 1;
                i += 1;
            }
        } else {
            // Invalid UTF-8 start byte (0xC0, 0xC1, 0xF5-0xFF)
            invalid += 1;
            i += 1;
        }
    }

    (valid, invalid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ascii() {
        let data = b"Hello World";
        let result = validate_utf8(data).unwrap();
        assert_eq!(result.encoding, Encoding::Utf8);
        assert!(result.confidence > 0.9);
    }

    #[test]
    fn test_valid_utf8_chinese() {
        // "你好世界" in UTF-8
        let data = vec![
            0xE4, 0xBD, 0xA0, 0xE5, 0xA5, 0xBD, 0xE4, 0xB8, 0x96, 0xE7, 0x95, 0x8C,
        ];
        let result = validate_utf8(&data).unwrap();
        assert_eq!(result.encoding, Encoding::Utf8);
        assert!(result.confidence > 0.9);
    }

    #[test]
    fn test_utf8_bom() {
        let mut data = vec![0xEF, 0xBB, 0xBF];
        data.extend(b"Hello");
        let result = validate_utf8(&data).unwrap();
        assert_eq!(result.encoding, Encoding::Utf8);
        assert!(result.confidence > 0.9);
        assert_eq!(result.method, DetectionMethod::Bom);
    }

    #[test]
    fn test_invalid_utf8() {
        // Invalid UTF-8 sequence
        let data = vec![0xFF, 0xFF, 0xFF];
        assert!(validate_utf8(&data).is_none());
    }

    #[test]
    fn test_overlong_encoding() {
        // Overlong encoding (should be detected as invalid)
        let data = vec![0xC0, 0x80]; // Overlong encoding of NULL
        let result = validate_utf8(&data);
        // Should either be None or have low confidence
        if let Some(r) = result {
            assert!(r.confidence < 0.8);
        }
    }
}
