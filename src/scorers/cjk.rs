/// CJK (Chinese, Japanese, Korean) encoding scorers
///
/// Implements detection for GBK, GB18030, Big5, Shift_JIS, CP949, EUC-JP, EUC-KR
use crate::models::{DetectionMethod, DetectionResult, Encoding, Language};
use crate::ranges::*;

// ============================================================================
// Chinese Discriminators (GBK/GB18030)
// ============================================================================

/// Common Chinese characters for GBK/GB18030 detection
const CHINESE_DISCRIMINATORS: &[(u8, u8)] = &[
    // Top 20 most common Chinese characters
    (0xB5, 0xC4), // 的
    (0xCA, 0xC7), // 是
    (0xD2, 0xBB), // 一
    (0xC1, 0xCB), // 了
    (0xD4, 0xDA), // 在
    (0xD3, 0xD0), // 有
    (0xBA, 0xCD), // 和
    (0xC8, 0xCB), // 人
    (0xD5, 0xE2), // 这
    (0xD6, 0xD0), // 中
    (0xB4, 0xF3), // 大
    (0xC0, 0xB4), // 来
    (0xC9, 0xCF), // 上
    (0xB8, 0xF6), // 个
    (0xB5, 0xBD), // 到
    (0xCB, 0xB5), // 说
    (0xC4, 0xE3), // 你
    (0xBA, 0xC3), // 好
    (0xCA, 0xC0), // 世
    (0xBD, 0xE7), // 界
    (0xCE, 0xD2), // 我
    (0xCB, 0xFB), // 他
    (0xC3, 0xC7), // 们
    (0xCA, 0xB1), // 时
    (0xBB, 0xE1), // 会
    (0xBF, 0xC9), // 可
    (0xD2, 0xD4), // 以
    (0xD0, 0xC7), // 为
    (0xB9, 0xFA), // 国
    (0xC4, 0xEA), // 作
    (0xB7, 0xD1), // 家
    (0xB7, 0xA2), // 发
    (0xB9, 0xA4), // 共
    (0xD4, 0xDA), // 自
];

/// Score byte sequence for Chinese (GBK/GB18030)
pub fn score_chinese(data: &[u8]) -> f32 {
    let mut matches = 0;
    let mut checked = 0;

    let mut i = 0;
    while i + 1 < data.len() {
        let lead = data[i];
        let trail = data[i + 1];

        if GBK_LEAD_BYTE.contains(&lead) && in_ranges(trail, GBK_TRAIL_BYTE) {
            checked += 1;
            if CHINESE_DISCRIMINATORS.contains(&(lead, trail)) {
                matches += 1;
            }
        }
        i += 2;
    }

    if checked == 0 {
        return 0.0;
    }
    matches as f32 / checked as f32
}

/// Check for GBK-specific patterns
pub fn has_gbk_patterns(data: &[u8]) -> bool {
    // Hard rule: 0x40 as trail byte is valid in GBK but not CP949
    let mut i = 0;
    while i + 1 < data.len() {
        let lead = data[i];
        let trail = data[i + 1];
        if GBK_LEAD_BYTE.contains(&lead) && trail == 0x40 {
            return true;
        }
        i += 2;
    }
    false
}

// ============================================================================
// Korean Discriminators (CP949/EUC-KR)
// ============================================================================

/// Common Korean characters for CP949/EUC-KR detection
const KOREAN_DISCRIMINATORS: &[(u8, u8)] = &[
    // Common Hangul syllables and particles
    (0xC0, 0xCC), // 이
    (0xB0, 0xDD), // 것
    (0xB5, 0xE9), // 들
    (0xB8, 0xA6), // 를
    (0xC0, 0xCE), // 의
    (0xBE, 0xC8), // 안
    (0xB3, 0xE7), // 녕
    (0xC7, 0xCF), // 하
    (0xBC, 0xBC), // 세
    (0xBF, 0xE4), // 요
    (0xC7, 0xD1), // 한
    (0xB1, 0xDB), // 글
    (0xC0, 0xFA), // 은
    (0xB4, 0xC2), // 는
    (0xB4, 0xD9), // 다
    (0xC0, 0xA1), // 아
    (0xC7, 0xCF), // 하
    (0xB8, 0xE9), // 슴
    (0xB4, 0xC2), // 니
    (0xC8, 0xA3), // 다
    (0xC7, 0xCF), // 합
    (0xB3, 0xB2), // 너
];

/// Score byte sequence for Korean (CP949/EUC-KR)
pub fn score_korean(data: &[u8]) -> f32 {
    let mut matches = 0;
    let mut checked = 0;

    let mut i = 0;
    while i + 1 < data.len() {
        let lead = data[i];
        let trail = data[i + 1];

        if CP949_LEAD_BYTE.contains(&lead) && in_ranges(trail, CP949_TRAIL_BYTE) {
            checked += 1;
            if KOREAN_DISCRIMINATORS.contains(&(lead, trail)) {
                matches += 1;
            }
        }
        i += 2;
    }

    if checked == 0 {
        return 0.0;
    }
    matches as f32 / checked as f32
}

// ============================================================================
// Big5 Discriminators (Traditional Chinese)
// ============================================================================

/// Common Traditional Chinese characters for Big5 detection
const BIG5_DISCRIMINATORS: &[(u8, u8)] = &[
    // Top 20 Traditional Chinese characters
    (0xA7, 0x41), // 的
    (0xA6, 0x7C), // 是
    (0xA4, 0x40), // 一
    (0xA4, 0xF3), // 了
    (0xA6, 0xB3), // 在
    (0xA6, 0xB3), // 有
    (0xA9, 0xD1), // 和
    (0xA4, 0xE8), // 人
    (0xB3, 0xA1), // 這
    (0xA4, 0xA4), // 中
    (0xA4, 0xF4), // 大
    (0xA5, 0xD5), // 來
    (0xA4, 0xA7), // 上
    (0xAD, 0xD6), // 個
    (0xA6, 0xDC), // 到
    (0xBB, 0xA1), // 說
    (0xA7, 0x51), // 你
    (0xA6, 0xB7), // 好
    (0xA5, 0x4A), // 世
    (0xAC, 0x43), // 界
];

/// Score byte sequence for Big5
pub fn score_big5(data: &[u8]) -> f32 {
    let mut matches = 0;
    let mut checked = 0;
    let mut valid_big5 = 0;

    let mut i = 0;
    while i + 1 < data.len() {
        let lead = data[i];
        let trail = data[i + 1];

        if BIG5_LEAD_BYTE.contains(&lead) && in_ranges(trail, BIG5_TRAIL_BYTE) {
            valid_big5 += 1;
            if BIG5_DISCRIMINATORS.contains(&(lead, trail)) {
                matches += 1;
            }
        }
        i += 2;
    }

    if valid_big5 == 0 {
        return 0.0;
    }

    // High score if we have many valid Big5 sequences
    let valid_ratio = valid_big5 as f32 / (data.len() / 2) as f32;
    let match_score = matches as f32 / valid_big5 as f32;

    valid_ratio * 0.5 + match_score * 0.5
}

// ============================================================================
// Shift_JIS Discriminators (Japanese)
// ============================================================================

/// Common Japanese characters for Shift_JIS detection
const SHIFT_JIS_DISCRIMINATORS: &[(u8, u8)] = &[
    // Hiragana (common)
    (0x82, 0xA0), // あ
    (0x82, 0xA2), // い
    (0x82, 0xA4), // う
    (0x82, 0xA6), // え
    (0x82, 0xA8), // お
    (0x82, 0xA9), // か
    (0x82, 0xAB), // き
    (0x82, 0xAD), // く
    (0x82, 0xAF), // け
    (0x82, 0xB1), // こ
    (0x82, 0xB3), // さ
    (0x82, 0xB5), // し
    (0x82, 0xB7), // す
    (0x82, 0xB9), // せ
    (0x82, 0xBB), // そ
    (0x82, 0xC1), // た
    (0x82, 0xC3), // ち
    (0x82, 0xC5), // つ
    (0x82, 0xC7), // て
    (0x82, 0xC9), // と
    // Common kanji
    (0x93, 0xFA), // 日
    (0x8C, 0xEA), // 本
    (0x8C, 0xEA), // 語
    (0x82, 0xD0), // は
];

/// Score byte sequence for Shift_JIS
pub fn score_shift_jis(data: &[u8]) -> f32 {
    let mut matches = 0;
    let mut checked = 0;
    let mut katakana_count = 0;
    let mut valid_sjis = 0;

    let mut i = 0;
    while i < data.len() {
        let byte = data[i];

        // Check for half-width Katakana (0xA1-0xDF)
        if SHIFT_JIS_KATAKANA.contains(&byte) {
            katakana_count += 1;
            i += 1;
            continue;
        }

        // Check for double-byte sequences
        if i + 1 < data.len() && in_ranges(byte, SHIFT_JIS_LEAD_BYTE) {
            let trail = data[i + 1];
            if in_ranges(trail, SHIFT_JIS_TRAIL_BYTE) {
                valid_sjis += 1;
                if SHIFT_JIS_DISCRIMINATORS.contains(&(byte, trail)) {
                    matches += 1;
                }
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    if valid_sjis == 0 {
        return 0.0;
    }

    // Shift_JIS has characteristic half-width Katakana
    let katakana_score = (katakana_count as f32 / data.len() as f32).min(0.3) * 2.0;
    let match_score = matches as f32 / valid_sjis as f32;

    katakana_score * 0.4 + match_score * 0.6
}

// ============================================================================
// Main CJK Detection Functions
// ============================================================================

/// Detect GBK/GB18030 encoding
pub fn detect_gbk(data: &[u8]) -> Option<DetectionResult> {
    // Hard rule check first
    if has_gbk_patterns(data) {
        return Some(
            DetectionResult::new(Encoding::Gbk, 0.95)
                .with_language(Language::Chinese)
                .with_method(DetectionMethod::HardRule),
        );
    }

    let chinese_score = score_chinese(data);

    if chinese_score > 0.3 {
        let confidence = (chinese_score * 0.9).min(0.92);
        Some(
            DetectionResult::new(Encoding::Gbk, confidence)
                .with_language(Language::Chinese)
                .with_method(DetectionMethod::FrequencyAnalysis),
        )
    } else {
        None
    }
}

/// Detect CP949 encoding
pub fn detect_cp949(data: &[u8]) -> Option<DetectionResult> {
    let korean_score = score_korean(data);

    if korean_score > 0.3 {
        let confidence = (korean_score * 0.9).min(0.92);
        Some(
            DetectionResult::new(Encoding::Cp949, confidence)
                .with_language(Language::Korean)
                .with_method(DetectionMethod::FrequencyAnalysis),
        )
    } else {
        None
    }
}

/// Detect Big5 encoding
pub fn detect_big5(data: &[u8]) -> Option<DetectionResult> {
    let big5_score = score_big5(data);

    if big5_score > 0.3 {
        let confidence = (big5_score * 0.9).min(0.90);
        Some(
            DetectionResult::new(Encoding::Big5, confidence)
                .with_language(Language::Chinese)
                .with_method(DetectionMethod::FrequencyAnalysis),
        )
    } else {
        None
    }
}

/// Detect Shift_JIS encoding
pub fn detect_shift_jis(data: &[u8]) -> Option<DetectionResult> {
    let sjis_score = score_shift_jis(data);

    if sjis_score > 0.25 {
        let confidence = (sjis_score * 0.9).min(0.90);
        Some(
            DetectionResult::new(Encoding::ShiftJis, confidence)
                .with_language(Language::Japanese)
                .with_method(DetectionMethod::FrequencyAnalysis),
        )
    } else {
        None
    }
}

/// Discriminate between GBK and CP949 (when both are possible)
pub fn discriminate_gbk_cp949(data: &[u8]) -> Option<DetectionResult> {
    // Hard rule: 0x40 trail byte means GBK
    if has_gbk_patterns(data) {
        return Some(
            DetectionResult::new(Encoding::Gbk, 0.95)
                .with_language(Language::Chinese)
                .with_method(DetectionMethod::HardRule),
        );
    }

    let chinese_score = score_chinese(data);
    let korean_score = score_korean(data);

    if chinese_score == 0.0 && korean_score == 0.0 {
        return None;
    }

    if chinese_score > korean_score {
        let confidence = if chinese_score + korean_score > 0.0 {
            chinese_score / (chinese_score + korean_score)
        } else {
            0.5
        };

        Some(
            DetectionResult::new(Encoding::Gbk, confidence.min(0.92))
                .with_language(Language::Chinese)
                .with_method(DetectionMethod::FrequencyAnalysis),
        )
    } else if korean_score > chinese_score {
        let confidence = if chinese_score + korean_score > 0.0 {
            korean_score / (chinese_score + korean_score)
        } else {
            0.5
        };

        Some(
            DetectionResult::new(Encoding::Cp949, confidence.min(0.92))
                .with_language(Language::Korean)
                .with_method(DetectionMethod::FrequencyAnalysis),
        )
    } else {
        // Tie - default to GBK (more common globally)
        Some(
            DetectionResult::new(Encoding::Gbk, 0.5)
                .with_language(Language::Chinese)
                .with_method(DetectionMethod::Fallback),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_chinese() {
        // "你好世界" in GBK
        let gbk = vec![0xC4, 0xE3, 0xBA, 0xC3, 0xCA, 0xC0, 0xBD, 0xE7];
        let score = score_chinese(&gbk);
        assert!(score > 0.3);
    }

    #[test]
    fn test_score_korean() {
        // "안녕하세요" in CP949
        let korean = vec![0xBE, 0xC8, 0xB3, 0xE7, 0xC7, 0xCF, 0xBC, 0xBC, 0xBF, 0xE4];
        let score = score_korean(&korean);
        assert!(score > 0.3);
    }

    #[test]
    fn test_gbk_hard_rule() {
        // Contains 0x40 as trail byte
        let data = vec![0x81, 0x40, 0xD2, 0xBB];
        assert!(has_gbk_patterns(&data));
    }

    #[test]
    fn test_discriminate_gbk_cp949() {
        // Chinese text
        let gbk = vec![0xC4, 0xE3, 0xBA, 0xC3, 0xCA, 0xC0, 0xBD, 0xE7];
        let result = discriminate_gbk_cp949(&gbk).unwrap();
        assert_eq!(result.encoding, Encoding::Gbk);

        // Korean text
        let korean = vec![0xBE, 0xC8, 0xB3, 0xE7, 0xC7, 0xCF, 0xBC, 0xBC, 0xBF, 0xE4];
        let result = discriminate_gbk_cp949(&korean).unwrap();
        assert_eq!(result.encoding, Encoding::Cp949);
    }

    #[test]
    fn test_detect_shift_jis() {
        // "こんにちは" in Shift_JIS
        let data = vec![
            0x82, 0xB1, 0x82, 0xF1, 0x82, 0xC9, 0x82, 0xBF, 0x82, 0xCD, 0x82, 0xCD, 0x82, 0xB5,
        ];
        let result = detect_shift_jis(&data);
        assert!(result.is_some());
        assert_eq!(result.unwrap().encoding, Encoding::ShiftJis);
    }
}
