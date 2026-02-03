/// Byte range definitions for various character encodings
///
/// This module defines the valid byte ranges for each encoding,
/// which is critical for accurate detection.
use std::ops::RangeInclusive;

/// Check if a byte is in the given inclusive range
#[inline]
pub const fn in_range(byte: u8, start: u8, end: u8) -> bool {
    byte >= start && byte <= end
}

/// Check if a byte is in any of the given ranges
#[inline]
pub fn in_ranges(byte: u8, ranges: &[RangeInclusive<u8>]) -> bool {
    ranges.iter().any(|range| range.contains(&byte))
}

// ============================================================================
// UTF-8 Ranges
// ============================================================================

/// UTF-8 continuation bytes range (0x80-0xBF)
pub const UTF8_CONTINUATION: RangeInclusive<u8> = 0x80..=0xBF;

/// Check if byte is a valid UTF-8 continuation byte
#[inline]
pub const fn is_utf8_continuation(byte: u8) -> bool {
    in_range(byte, 0x80, 0xBF)
}

// ============================================================================
// UTF-16 Ranges
// ============================================================================

/// UTF-16 high surrogate range (0xD800-0xDBFF)
pub const UTF16_HIGH_SURROGATE: RangeInclusive<u16> = 0xD800..=0xDBFF;

/// UTF-16 low surrogate range (0xDC00-0xDFFF)
pub const UTF16_LOW_SURROGATE: RangeInclusive<u16> = 0xDC00..=0xDFFF;

/// Check if code point is a high surrogate
#[inline]
pub const fn is_utf16_high_surrogate(cp: u16) -> bool {
    cp >= 0xD800 && cp <= 0xDBFF
}

/// Check if code point is a low surrogate
#[inline]
pub const fn is_utf16_low_surrogate(cp: u16) -> bool {
    cp >= 0xDC00 && cp <= 0xDFFF
}

// ============================================================================
// GBK / GB18030 Ranges
// ============================================================================

/// GBK lead byte range (0x81-0xFE)
pub const GBK_LEAD_BYTE: RangeInclusive<u8> = 0x81..=0xFE;

/// GBK trail byte ranges (0x40-0x7E, 0x80-0xFE)
pub const GBK_TRAIL_BYTE: &[RangeInclusive<u8>] = &[0x40..=0x7E, 0x80..=0xFE];

/// GB18030 4-byte sequence ranges
pub const GB18030_4BYTE_FIRST: RangeInclusive<u8> = 0x81..=0xFE;
pub const GB18030_4BYTE_SECOND: RangeInclusive<u8> = 0x30..=0x39;
pub const GB18030_4BYTE_THIRD: RangeInclusive<u8> = 0x81..=0xFE;
pub const GB18030_4BYTE_FOURTH: RangeInclusive<u8> = 0x30..=0x39;

// ============================================================================
// CP949 Ranges
// ============================================================================

/// CP949 lead byte range (0x81-0xFE) - same as GBK!
pub const CP949_LEAD_BYTE: RangeInclusive<u8> = 0x81..=0xFE;

/// CP949 trail byte ranges (0x41-0x5A, 0x61-0x7A, 0x81-0xFE)
/// Note: CP949 EXCLUDES 0x40 in trail byte position (hard discrimination rule!)
pub const CP949_TRAIL_BYTE: &[RangeInclusive<u8>] = &[0x41..=0x5A, 0x61..=0x7A, 0x81..=0xFE];

// ============================================================================
// Big5 Ranges
// ============================================================================

/// Big5 lead byte range (0x81-0xFE)
pub const BIG5_LEAD_BYTE: RangeInclusive<u8> = 0x81..=0xFE;

/// Big5 trail byte range (0x40-0x7E, 0xA1-0xFE)
pub const BIG5_TRAIL_BYTE: &[RangeInclusive<u8>] = &[0x40..=0x7E, 0xA1..=0xFE];

// ============================================================================
// Shift_JIS Ranges
// ============================================================================

/// Shift_JIS lead byte range (0x81-0x9F, 0xE0-0xEF)
pub const SHIFT_JIS_LEAD_BYTE: &[RangeInclusive<u8>] = &[0x81..=0x9F, 0xE0..=0xEF];

/// Shift_JIS trail byte range (0x40-0xFC, excluding 0x7F)
pub const SHIFT_JIS_TRAIL_BYTE: &[RangeInclusive<u8>] = &[0x40..=0x7E, 0x80..=0xFC];

/// Shift_JIS single-byte Katakana range (0xA1-0xDF)
pub const SHIFT_JIS_KATAKANA: RangeInclusive<u8> = 0xA1..=0xDF;

// ============================================================================
// EUC-JP Ranges
// ============================================================================

/// EUC-JP lead byte range (0x8E, 0x8F, 0xA1-0xFE)
pub const EUC_JP_LEAD_BYTE: &[RangeInclusive<u8>] = &[
    0x8E..=0x8E, // Half-width Katakana prefix
    0x8F..=0x8F, // JIS X 0212 prefix
    0xA1..=0xFE, // JIS X 0208
];

/// EUC-JP trail byte range (0xA1-0xFE)
pub const EUC_JP_TRAIL_BYTE: RangeInclusive<u8> = 0xA1..=0xFE;

/// EUC-JP half-width Katakana range (0xA1-0xDF)
pub const EUC_JP_HALF_KATAKANA: RangeInclusive<u8> = 0xA1..=0xDF;

// ============================================================================
// EUC-KR Ranges
// ============================================================================

/// EUC-KR lead byte range (0x81-0xFE)
pub const EUC_KR_LEAD_BYTE: RangeInclusive<u8> = 0x81..=0xFE;

/// EUC-KR trail byte range (0x41-0x5A, 0x61-0x7A, 0x81-0xFE)
pub const EUC_KR_TRAIL_BYTE: &[RangeInclusive<u8>] = &[0x41..=0x5A, 0x61..=0x7A, 0x81..=0xFE];

// ============================================================================
// ISO-2022 Escape Sequences
// ============================================================================

/// ISO-2022-JP escape sequences
pub const ISO2022_JP_SEQUENCES: &[&[u8]] = &[
    b"\x1b(B", // ASCII
    b"\x1b(J", // Roman
    b"\x1b$B", // JIS X 0208
    b"\x1b$@", // JIS X 0208 (old)
    b"\x1b$(", // JIS X 0208 (variant)
    b"\x1b(I", // Half-width Katakana
];

/// ISO-2022-KR escape sequences
pub const ISO2022_KR_SEQUENCES: &[&[u8]] = &[
    b"\x1b$)C", // KS X 1001
];

// ============================================================================
// Helper Functions
// ============================================================================

/// Check if bytes contain ISO-2022 escape sequences
pub fn has_iso2022_sequences(data: &[u8]) -> bool {
    const ISO2022_ALL: &[&[u8]] = &[
        b"\x1b(B", b"\x1b(J", b"\x1b$B", b"\x1b$@", b"\x1b$(", b"\x1b(I", b"\x1b$)C",
    ];

    ISO2022_ALL
        .iter()
        .any(|seq| data.windows(seq.len()).any(|window| window == *seq))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_range() {
        assert!(in_range(0x81, 0x80, 0x90));
        assert!(!in_range(0x7F, 0x80, 0x90));
    }

    #[test]
    fn test_utf8_continuation() {
        assert!(is_utf8_continuation(0x80));
        assert!(is_utf8_continuation(0xBF));
        assert!(!is_utf8_continuation(0x7F));
        assert!(!is_utf8_continuation(0xC0));
    }

    #[test]
    fn test_utf16_surrogates() {
        assert!(is_utf16_high_surrogate(0xD800));
        assert!(is_utf16_high_surrogate(0xDBFF));
        assert!(!is_utf16_high_surrogate(0xD7FF));

        assert!(is_utf16_low_surrogate(0xDC00));
        assert!(is_utf16_low_surrogate(0xDFFF));
        assert!(!is_utf16_low_surrogate(0xDBFF));
    }

    #[test]
    fn test_gbk_ranges() {
        assert!(GBK_LEAD_BYTE.contains(&0x81));
        assert!(GBK_LEAD_BYTE.contains(&0xFE));

        assert!(in_ranges(0x40, GBK_TRAIL_BYTE));
        assert!(in_ranges(0x7E, GBK_TRAIL_BYTE));
        assert!(in_ranges(0x80, GBK_TRAIL_BYTE));
        assert!(!in_ranges(0x7F, GBK_TRAIL_BYTE));
    }

    #[test]
    fn test_cp949_excludes_0x40() {
        // CRITICAL: CP949 excludes 0x40 as trail byte (hard rule!)
        assert!(!in_ranges(0x40, CP949_TRAIL_BYTE));
        assert!(in_ranges(0x41, CP949_TRAIL_BYTE));
    }

    #[test]
    fn test_big5_ranges() {
        assert!(BIG5_LEAD_BYTE.contains(&0x81));
        assert!(in_ranges(0x40, BIG5_TRAIL_BYTE));
        assert!(in_ranges(0xA1, BIG5_TRAIL_BYTE));
    }

    #[test]
    fn test_shift_jis_ranges() {
        assert!(in_ranges(0x81, SHIFT_JIS_LEAD_BYTE));
        assert!(in_ranges(0x9F, SHIFT_JIS_LEAD_BYTE));
        assert!(in_ranges(0xE0, SHIFT_JIS_LEAD_BYTE));

        assert!(SHIFT_JIS_KATAKANA.contains(&0xA1));
        assert!(SHIFT_JIS_KATAKANA.contains(&0xDF));
    }

    #[test]
    fn test_iso2022_sequences() {
        let data = b"Hello \x1b$B\x22\x44\x1b(B World";
        assert!(has_iso2022_sequences(data));

        let plain = b"Just ASCII text";
        assert!(!has_iso2022_sequences(plain));
    }
}
