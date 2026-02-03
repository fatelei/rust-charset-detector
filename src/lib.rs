//! High-performance character encoding detection library
//!
//! This library provides fast and accurate charset encoding detection,
//! with special focus on distinguishing between overlapping encodings
//! such as GBK vs CP949.
//!
//! ## Features
//!
//! - Fast UTF-8 validation
//! - CJK encoding detection (GBK, GB18030, Big5, Shift_JIS, CP949, etc.)
//! - BOM detection for UTF-16/UTF-32
//! - ISO-2022 escape sequence detection
//! - Python bindings via PyO3
//!
//! ## Example
//!
//! ```rust
//! use charset_detector::{detect, detect_best};
//!
//! let data = b"Hello, world!";
//! let results = detect(data);
//! if let Some(best) = detect_best(data) {
//!     println!("Detected: {} with {:.2}% confidence",
//!              best.encoding, best.confidence * 100.0);
//! }
//! ```

pub mod detector;
pub mod models;
pub mod ranges;
pub mod scorers;

// Re-export main types for convenience
pub use detector::{detect, detect_best, detect_encoding, CharsetDetector};
pub use models::{DetectionMethod, DetectionResult, DetectionResults, Encoding, Language};

/// Python bindings (only available when `python` feature is enabled)
#[cfg(feature = "python")]
pub mod python;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_detect_utf8() {
        let data = "Hello, 世界!".as_bytes();
        let result = detect_best(data).unwrap();
        assert_eq!(result.encoding, Encoding::Utf8);
    }
}
