// Python bindings via PyO3 0.22+
//
// Provides a Python-friendly API compatible with charset_normalizer

use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::detector::CharsetDetector;
use crate::models::DetectionResult as RustDetectionResult;

/// Helper function to convert PyBytes to Vec<u8>
fn pybytes_to_vec(data: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
    // Try to get bytes from Python object
    if let Ok(bytes) = data.downcast::<PyBytes>() {
        // Use the bytes method to get a Python bytes object
        Ok(bytes.as_bytes().to_vec())
    } else {
        // Fallback: try to get bytes-like object
        let py_bytes = data.call_method0("__bytes__")?;
        let bytes_ref = py_bytes.downcast::<PyBytes>()?;
        Ok(bytes_ref.as_bytes().to_vec())
    }
}

/// Python wrapper for DetectionResult
#[pyclass(name = "CharsetMatch")]
#[derive(Clone)]
pub struct PyCharsetMatch {
    pub(crate) result: RustDetectionResult,
}

#[pymethods]
impl PyCharsetMatch {
    /// Get the detected encoding (IANA name)
    #[getter]
    fn encoding(&self) -> String {
        self.result.encoding.iana_name().to_string()
    }

    /// Get the Python encoding name (for codecs module)
    #[getter]
    fn encoding_iana(&self) -> String {
        self.result.encoding.iana_name().to_string()
    }

    /// Get confidence score (0.0 to 1.0)
    #[getter]
    fn confidence(&self) -> f32 {
        self.result.confidence
    }

    /// Get language code (e.g., "zh", "ja", "ko", "en")
    #[getter]
    fn language(&self) -> Option<String> {
        self.result.language.map(|l| l.to_string())
    }

    /// Get the detection method used
    #[getter]
    fn method(&self) -> String {
        self.result.method.to_string()
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "CharsetMatch(encoding='{}', confidence={:.2}, language={})",
            self.result.encoding,
            self.result.confidence,
            self.result
                .language
                .map_or("None".to_string(), |l| l.to_string())
        )
    }

    /// String representation
    fn __str__(&self) -> String {
        self.__repr__()
    }
}

/// Python wrapper for DetectionResults (multiple matches)
#[pyclass(name = "CharsetMatches")]
pub struct PyCharsetMatches {
    pub(crate) results: Vec<PyCharsetMatch>,
}

#[pymethods]
impl PyCharsetMatches {
    /// Get the best match
    #[getter]
    fn best(&self) -> Option<PyCharsetMatch> {
        self.results.first().cloned()
    }

    /// Get all matches
    fn all(&self) -> Vec<PyCharsetMatch> {
        self.results.clone()
    }

    /// Number of candidates
    fn __len__(&self) -> usize {
        self.results.len()
    }

    /// Iterate over matches
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// Get next match (for iteration)
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyCharsetMatch> {
        if slf.results.is_empty() {
            None
        } else {
            Some(slf.results.remove(0))
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        if let Some(best) = self.best() {
            format!(
                "CharsetMatches(best='{}', {} candidates)",
                best.encoding(),
                self.results.len()
            )
        } else {
            "CharsetMatches(no candidates)".to_string()
        }
    }
}

/// Main charset detector class
#[pyclass(name = "CharsetDetector")]
pub struct PyCharsetDetector {
    detector: CharsetDetector,
}

#[pymethods]
impl PyCharsetDetector {
    /// Create a new detector
    #[new]
    #[pyo3(signature = (min_confidence=0.3))]
    fn new(min_confidence: f32) -> Self {
        Self {
            detector: CharsetDetector::new().with_min_confidence(min_confidence),
        }
    }

    /// Detect charset from bytes
    fn detect(&self, data: &Bound<'_, PyAny>) -> PyResult<PyCharsetMatches> {
        let bytes = pybytes_to_vec(data)?;
        let rust_results = self.detector.detect(&bytes);

        let results: Vec<PyCharsetMatch> = rust_results
            .candidates
            .into_iter()
            .map(|r| PyCharsetMatch { result: r })
            .collect();

        Ok(PyCharsetMatches { results })
    }

    /// Detect and return only the best match
    fn detect_best(&self, data: &Bound<'_, PyAny>) -> PyResult<Option<PyCharsetMatch>> {
        let bytes = pybytes_to_vec(data)?;
        Ok(self
            .detector
            .detect_best(&bytes)
            .map(|r| PyCharsetMatch { result: r }))
    }

    /// Detect and return encoding name only (convenience method)
    fn detect_encoding(&self, data: &Bound<'_, PyAny>) -> PyResult<Option<String>> {
        let bytes = pybytes_to_vec(data)?;
        Ok(self
            .detector
            .detect_encoding(&bytes)
            .map(|e| e.iana_name().to_string()))
    }

    /// String representation
    fn __repr__(&self) -> String {
        "CharsetDetector()".to_string()
    }
}

/// Convenience function: detect charset from bytes
#[pyfunction]
#[pyo3(signature = (data, min_confidence=0.3))]
fn detect_from_bytes(data: &Bound<'_, PyAny>, min_confidence: f32) -> PyResult<PyCharsetMatches> {
    let detector = CharsetDetector::new().with_min_confidence(min_confidence);
    let bytes = pybytes_to_vec(data)?;
    let rust_results = detector.detect(&bytes);

    let results: Vec<PyCharsetMatch> = rust_results
        .candidates
        .into_iter()
        .map(|r| PyCharsetMatch { result: r })
        .collect();

    Ok(PyCharsetMatches { results })
}

/// Convenience function: detect and return best match only
#[pyfunction]
#[pyo3(signature = (data, min_confidence=0.3))]
fn detect_best_from_bytes(
    data: &Bound<'_, PyAny>,
    min_confidence: f32,
) -> PyResult<Option<PyCharsetMatch>> {
    let detector = CharsetDetector::new().with_min_confidence(min_confidence);
    let bytes = pybytes_to_vec(data)?;
    Ok(detector
        .detect_best(&bytes)
        .map(|r| PyCharsetMatch { result: r }))
}

/// Python module definition
#[pymodule]
fn _detector(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCharsetDetector>()?;
    m.add_class::<PyCharsetMatch>()?;
    m.add_class::<PyCharsetMatches>()?;
    m.add_function(wrap_pyfunction!(detect_from_bytes, m)?)?;
    m.add_function(wrap_pyfunction!(detect_best_from_bytes, m)?)?;

    // Add module-level constants
    m.add("VERSION", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
