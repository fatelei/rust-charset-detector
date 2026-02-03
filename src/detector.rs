/// Main charset detection engine
///
/// Orchestrates all detection strategies and provides the main API

use crate::models::{DetectionResult, DetectionResults, Encoding, Language};
use crate::ranges::has_iso2022_sequences;
use crate::scorers::{validate_utf8, detect_gbk, detect_cp949, detect_big5, detect_shift_jis};

/// Main charset detector
pub struct CharsetDetector {
    /// Minimum confidence threshold
    min_confidence: f32,
}

impl CharsetDetector {
    /// Create a new detector with default settings
    pub fn new() -> Self {
        Self {
            min_confidence: 0.3,
        }
    }

    /// Set minimum confidence threshold
    pub fn with_min_confidence(mut self, threshold: f32) -> Self {
        self.min_confidence = threshold.clamp(0.0, 1.0);
        self
    }

    /// Detect charset from raw bytes
    pub fn detect(&self, data: &[u8]) -> DetectionResults {
        let mut results = DetectionResults::new();

        if data.is_empty() {
            // Empty data - return UTF-8 with low confidence
            results.add(DetectionResult::new(Encoding::Utf8, 0.1));
            return results;
        }

        // Phase 1: Check for BOM (Byte Order Mark)
        if let Some(result) = detect_bom(data) {
            results.add(result);
            return results;
        }

        // Phase 2: UTF-8 validation (fast and reliable)
        if let Some(result) = validate_utf8(data) {
            if result.confidence >= 0.9 {
                // High confidence UTF-8 - return immediately
                results.add(result);
                return results;
            }
            results.add(result);
        }

        // Phase 3: Check for ISO-2022 escape sequences
        if has_iso2022_sequences(data) {
            // Could be ISO-2022-JP or ISO-2022-KR
            // Add with medium confidence
            results.add(DetectionResult::new(Encoding::Iso2022Jp, 0.7)
                .with_language(Language::Japanese));
            results.add(DetectionResult::new(Encoding::Iso2022Kr, 0.6)
                .with_language(Language::Korean));
        }

        // Phase 4: CJK encoding detection
        // Try all CJK encodings and collect results

        // GBK vs CP949 discrimination
        let gbk_cp949_result = crate::scorers::discriminate_gbk_cp949(data);
        if let Some(ref result) = gbk_cp949_result {
            results.add(result.clone());
        }

        // Other CJK encodings
        if let Some(result) = detect_big5(data) {
            results.add(result);
        }

        if let Some(result) = detect_shift_jis(data) {
            results.add(result);
        }

        // Phase 5: If we have high confidence results, filter them
        let confident: Vec<_> = results.confident(0.7).into_iter().cloned().collect();

        if !confident.is_empty() {
            // Return only confident results
            let mut filtered = DetectionResults::new();
            for result in confident {
                filtered.add(result);
            }
            return filtered;
        }

        // Phase 6: If no confident results, return all candidates
        if results.is_empty() {
            // Fallback - assume UTF-8 or Latin-1
            results.add(DetectionResult::new(Encoding::Utf8, 0.3));
            results.add(DetectionResult::new(Encoding::Iso8859_1, 0.2));
        }

        results
    }

    /// Detect charset and return only the best match
    pub fn detect_best(&self, data: &[u8]) -> Option<DetectionResult> {
        self.detect(data).best().cloned()
    }

    /// Detect charset and return the encoding name (convenience method)
    pub fn detect_encoding(&self, data: &[u8]) -> Option<Encoding> {
        self.detect_best(data).map(|r| r.encoding)
    }

    /// Detect charset and return the IANA name (for compatibility)
    pub fn detect_iana(&self, data: &[u8]) -> Option<&'static str> {
        self.detect_encoding(data).map(|e| e.iana_name())
    }
}

impl Default for CharsetDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Detect BOM (Byte Order Mark)
fn detect_bom(data: &[u8]) -> Option<DetectionResult> {
    if data.len() >= 3 {
        match &data[0..3] {
            b"\xEF\xBB\xBF" => {
                return Some(DetectionResult::new(Encoding::Utf8, 0.99)
                    .with_method(crate::models::DetectionMethod::Bom));
            }
            _ => {}
        }
    }

    if data.len() >= 2 {
        match &data[0..2] {
            b"\xFF\xFE" => {
                return Some(DetectionResult::new(Encoding::Utf16Le, 0.99)
                    .with_method(crate::models::DetectionMethod::Bom));
            }
            b"\xFE\xFF" => {
                return Some(DetectionResult::new(Encoding::Utf16Be, 0.99)
                    .with_method(crate::models::DetectionMethod::Bom));
            }
            _ => {}
        }
    }

    if data.len() >= 4 {
        match &data[0..4] {
            b"\xFF\xFE\x00\x00" => {
                return Some(DetectionResult::new(Encoding::Utf32Le, 0.99)
                    .with_method(crate::models::DetectionMethod::Bom));
            }
            b"\x00\x00\xFE\xFF" => {
                return Some(DetectionResult::new(Encoding::Utf32Be, 0.99)
                    .with_method(crate::models::DetectionMethod::Bom));
            }
            _ => {}
        }
    }

    None
}

/// Convenience function: detect charset from bytes
pub fn detect(data: &[u8]) -> DetectionResults {
    CharsetDetector::new().detect(data)
}

/// Convenience function: detect best matching charset
pub fn detect_best(data: &[u8]) -> Option<DetectionResult> {
    CharsetDetector::new().detect_best(data)
}

/// Convenience function: detect encoding only
pub fn detect_encoding(data: &[u8]) -> Option<Encoding> {
    CharsetDetector::new().detect_encoding(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_utf8() {
        let data = b"Hello, world!";
        let result = detect_best(&data[..]).unwrap();
        assert_eq!(result.encoding, Encoding::Utf8);
    }

    #[test]
    fn test_detect_utf8_bom() {
        let mut data = vec![0xEF, 0xBB, 0xBF];
        data.extend_from_slice(b"Hello");
        let result = detect_best(&data).unwrap();
        assert_eq!(result.encoding, Encoding::Utf8);
        assert!(result.confidence > 0.9);
    }

    #[test]
    fn test_detect_gbk() {
        // "你好世界" in GBK
        let data = vec![0xC4, 0xE3, 0xBA, 0xC3, 0xCA, 0xC0, 0xBD, 0xE7];
        let result = detect_best(&data).unwrap();
        assert_eq!(result.encoding, Encoding::Gbk);
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_detect_cp949() {
        // "안녕하세요" in CP949
        let data = vec![0xBE, 0xC8, 0xB3, 0xE7, 0xC7, 0xCF, 0xBC, 0xBC, 0xBF, 0xE4];
        let result = detect_best(&data).unwrap();
        assert_eq!(result.encoding, Encoding::Cp949);
        assert!(result.confidence > 0.5);
    }

    #[test]
    fn test_empty_data() {
        let data = b"";
        let results = detect(data);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_multiple_results() {
        // ASCII could be UTF-8 or any single-byte encoding
        let data = b"Hello";
        let results = detect(data);
        assert!(results.len() >= 1);
    }
}
