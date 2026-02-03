/// Core data models for charset detection
///
/// Defines encoding types, detection results, and related structures

use std::fmt;

/// Supported character encodings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Encoding {
    // Unicode encodings
    Utf8,
    Utf16Le,
    Utf16Be,
    Utf32Le,
    Utf32Be,

    // CJK encodings
    Gb18030,
    Gbk,
    Big5,
    ShiftJis,
    Cp949,
    EucJp,
    EucKr,
    Iso2022Jp,
    Iso2022Kr,

    // Single-byte encodings (ISO-8859 series)
    Iso8859_1,
    Iso8859_2,
    Iso8859_3,
    Iso8859_4,
    Iso8859_5,
    Iso8859_6,
    Iso8859_7,
    Iso8859_8,
    Iso8859_9,
    Iso8859_10,
    Iso8859_11,
    Iso8859_13,
    Iso8859_14,
    Iso8859_15,
    Iso8859_16,

    // Windows code pages
    Windows1250,
    Windows1251,
    Windows1252,
    Windows1253,
    Windows1254,
    Windows1255,
    Windows1256,
    Windows1257,
    Windows1258,

    // Other encodings
    MacRoman,
    Ibm437,
    Ibm850,

    // Fallback
    Unknown,
}

impl Encoding {
    /// Get the IANA charset name for this encoding
    pub fn iana_name(&self) -> &'static str {
        match self {
            Encoding::Utf8 => "UTF-8",
            Encoding::Utf16Le => "UTF-16LE",
            Encoding::Utf16Be => "UTF-16BE",
            Encoding::Utf32Le => "UTF-32LE",
            Encoding::Utf32Be => "UTF-32BE",
            Encoding::Gb18030 => "GB18030",
            Encoding::Gbk => "GBK",
            Encoding::Big5 => "Big5",
            Encoding::ShiftJis => "Shift_JIS",
            Encoding::Cp949 => "CP949",
            Encoding::EucJp => "EUC-JP",
            Encoding::EucKr => "EUC-KR",
            Encoding::Iso2022Jp => "ISO-2022-JP",
            Encoding::Iso2022Kr => "ISO-2022-KR",
            Encoding::Iso8859_1 => "ISO-8859-1",
            Encoding::Iso8859_2 => "ISO-8859-2",
            Encoding::Iso8859_3 => "ISO-8859-3",
            Encoding::Iso8859_4 => "ISO-8859-4",
            Encoding::Iso8859_5 => "ISO-8859-5",
            Encoding::Iso8859_6 => "ISO-8859-6",
            Encoding::Iso8859_7 => "ISO-8859-7",
            Encoding::Iso8859_8 => "ISO-8859-8",
            Encoding::Iso8859_9 => "ISO-8859-9",
            Encoding::Iso8859_10 => "ISO-8859-10",
            Encoding::Iso8859_11 => "ISO-8859-11",
            Encoding::Iso8859_13 => "ISO-8859-13",
            Encoding::Iso8859_14 => "ISO-8859-14",
            Encoding::Iso8859_15 => "ISO-8859-15",
            Encoding::Iso8859_16 => "ISO-8859-16",
            Encoding::Windows1250 => "windows-1250",
            Encoding::Windows1251 => "windows-1251",
            Encoding::Windows1252 => "windows-1252",
            Encoding::Windows1253 => "windows-1253",
            Encoding::Windows1254 => "windows-1254",
            Encoding::Windows1255 => "windows-1255",
            Encoding::Windows1256 => "windows-1256",
            Encoding::Windows1257 => "windows-1257",
            Encoding::Windows1258 => "windows-1258",
            Encoding::MacRoman => "macintosh",
            Encoding::Ibm437 => "IBM437",
            Encoding::Ibm850 => "IBM850",
            Encoding::Unknown => "unknown",
        }
    }

    /// Get the Python encoding name (for codecs module)
    pub fn python_name(&self) -> &'static str {
        match self {
            Encoding::Utf8 => "utf_8",
            Encoding::Utf16Le => "utf_16_le",
            Encoding::Utf16Be => "utf_16_be",
            Encoding::Utf32Le => "utf_32_le",
            Encoding::Utf32Be => "utf_32_be",
            Encoding::Gb18030 => "gb18030",
            Encoding::Gbk => "gbk",
            Encoding::Big5 => "big5",
            Encoding::ShiftJis => "shift_jis",
            Encoding::Cp949 => "cp949",
            Encoding::EucJp => "euc_jp",
            Encoding::EucKr => "euc_kr",
            Encoding::Iso2022Jp => "iso2022_jp",
            Encoding::Iso2022Kr => "iso2022_kr",
            Encoding::Iso8859_1 => "latin_1",
            Encoding::Iso8859_2 => "iso8859_2",
            Encoding::Iso8859_3 => "iso8859_3",
            Encoding::Iso8859_4 => "iso8859_4",
            Encoding::Iso8859_5 => "iso8859_5",
            Encoding::Iso8859_6 => "iso8859_6",
            Encoding::Iso8859_7 => "iso8859_7",
            Encoding::Iso8859_8 => "iso8859_8",
            Encoding::Iso8859_9 => "iso8859_9",
            Encoding::Iso8859_10 => "iso8859_10",
            Encoding::Iso8859_11 => "iso8859_11",
            Encoding::Iso8859_13 => "iso8859_13",
            Encoding::Iso8859_14 => "iso8859_14",
            Encoding::Iso8859_15 => "iso8859_15",
            Encoding::Iso8859_16 => "iso8859_16",
            Encoding::Windows1250 => "cp1250",
            Encoding::Windows1251 => "cp1251",
            Encoding::Windows1252 => "cp1252",
            Encoding::Windows1253 => "cp1253",
            Encoding::Windows1254 => "cp1254",
            Encoding::Windows1255 => "cp1255",
            Encoding::Windows1256 => "cp1256",
            Encoding::Windows1257 => "cp1257",
            Encoding::Windows1258 => "cp1258",
            Encoding::MacRoman => "mac_roman",
            Encoding::Ibm437 => "cp437",
            Encoding::Ibm850 => "cp850",
            Encoding::Unknown => "unknown",
        }
    }

    /// Check if this is a Unicode encoding
    pub fn is_unicode(&self) -> bool {
        matches!(
            self,
            Encoding::Utf8 | Encoding::Utf16Le | Encoding::Utf16Be |
            Encoding::Utf32Le | Encoding::Utf32Be
        )
    }

    /// Check if this is a multi-byte encoding
    pub fn is_multibyte(&self) -> bool {
        self.is_unicode() || matches!(
            self,
            Encoding::Gb18030 | Encoding::Gbk | Encoding::Big5 |
            Encoding::ShiftJis | Encoding::Cp949 | Encoding::EucJp |
            Encoding::EucKr | Encoding::Iso2022Jp | Encoding::Iso2022Kr
        )
    }
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.iana_name())
    }
}

/// Language detection results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Chinese,
    Japanese,
    Korean,
    Thai,
    Arabic,
    Hebrew,
    Greek,
    Russian,
    English,
    Other,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Language::Chinese => "zh",
            Language::Japanese => "ja",
            Language::Korean => "ko",
            Language::Thai => "th",
            Language::Arabic => "ar",
            Language::Hebrew => "he",
            Language::Greek => "el",
            Language::Russian => "ru",
            Language::English => "en",
            Language::Other => "und",
        };
        write!(f, "{}", name)
    }
}

/// Detection result for a single encoding candidate
#[derive(Debug, Clone)]
pub struct DetectionResult {
    /// The detected encoding
    pub encoding: Encoding,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// Detected language (if applicable)
    pub language: Option<Language>,
    /// The method used for detection
    pub method: DetectionMethod,
}

/// Method used to detect the encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectionMethod {
    /// BOM (Byte Order Mark) detection
    Bom,
    /// UTF-8 validation
    Utf8Validation,
    /// Character frequency analysis
    FrequencyAnalysis,
    /// N-gram analysis
    NGramAnalysis,
    /// Byte range validation
    ByteRange,
    /// Hard rule (e.g., CP949 excludes 0x40 as trail byte)
    HardRule,
    /// Chaos/entropy measurement
    Chaos,
    /// Default fallback
    Fallback,
}

impl fmt::Display for DetectionMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            DetectionMethod::Bom => "bom",
            DetectionMethod::Utf8Validation => "utf8_validation",
            DetectionMethod::FrequencyAnalysis => "frequency_analysis",
            DetectionMethod::NGramAnalysis => "ngram_analysis",
            DetectionMethod::ByteRange => "byte_range",
            DetectionMethod::HardRule => "hard_rule",
            DetectionMethod::Chaos => "chaos",
            DetectionMethod::Fallback => "fallback",
        };
        write!(f, "{}", name)
    }
}

impl DetectionResult {
    /// Create a new detection result
    pub fn new(encoding: Encoding, confidence: f32) -> Self {
        Self {
            encoding,
            confidence: confidence.clamp(0.0, 1.0),
            language: None,
            method: DetectionMethod::Fallback,
        }
    }

    /// Set the language
    pub fn with_language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    /// Set the detection method
    pub fn with_method(mut self, method: DetectionMethod) -> Self {
        self.method = method;
        self
    }

    /// Check if confidence is above a threshold
    pub fn is_confident(&self, threshold: f32) -> bool {
        self.confidence >= threshold
    }
}

/// Multiple detection results (when there's ambiguity)
#[derive(Debug, Clone)]
pub struct DetectionResults {
    /// All candidates ranked by confidence
    pub candidates: Vec<DetectionResult>,
}

impl DetectionResults {
    /// Create empty results
    pub fn new() -> Self {
        Self {
            candidates: Vec::new(),
        }
    }

    /// Add a candidate
    pub fn add(&mut self, result: DetectionResult) {
        self.candidates.push(result);
        // Keep sorted by confidence (descending)
        self.candidates.sort_by(|a, b| {
            b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    /// Get the best match
    pub fn best(&self) -> Option<&DetectionResult> {
        self.candidates.first()
    }

    /// Get only confident results (above threshold)
    pub fn confident(&self, threshold: f32) -> Vec<&DetectionResult> {
        self.candidates.iter()
            .filter(|r| r.confidence >= threshold)
            .collect()
    }

    /// Number of candidates
    pub fn len(&self) -> usize {
        self.candidates.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.candidates.is_empty()
    }
}

impl Default for DetectionResults {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for DetectionResults {
    type Item = DetectionResult;
    type IntoIter = std::vec::IntoIter<DetectionResult>;

    fn into_iter(self) -> Self::IntoIter {
        self.candidates.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding_names() {
        assert_eq!(Encoding::Utf8.iana_name(), "UTF-8");
        assert_eq!(Encoding::Gbk.iana_name(), "GBK");
        assert_eq!(Encoding::ShiftJis.iana_name(), "Shift_JIS");
    }

    #[test]
    fn test_encoding_properties() {
        assert!(Encoding::Utf8.is_unicode());
        assert!(Encoding::Utf8.is_multibyte());
        assert!(Encoding::Gbk.is_multibyte());
        assert!(!Encoding::Iso8859_1.is_multibyte());
    }

    #[test]
    fn test_detection_result() {
        let result = DetectionResult::new(Encoding::Utf8, 0.95)
            .with_language(Language::English)
            .with_method(DetectionMethod::Bom);

        assert_eq!(result.encoding, Encoding::Utf8);
        assert_eq!(result.confidence, 0.95);
        assert!(result.is_confident(0.9));
        assert!(!result.is_confident(0.99));
    }

    #[test]
    fn test_detection_results() {
        let mut results = DetectionResults::new();
        results.add(DetectionResult::new(Encoding::Utf8, 0.8));
        results.add(DetectionResult::new(Encoding::Gbk, 0.95));
        results.add(DetectionResult::new(Encoding::Utf16Le, 0.7));

        assert_eq!(results.best().map(|r| r.encoding), Some(Encoding::Gbk));
        assert_eq!(results.len(), 3);
    }
}
