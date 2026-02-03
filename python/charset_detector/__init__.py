"""
High-performance character encoding detection library.

This library provides fast and accurate charset encoding detection,
with special focus on distinguishing between overlapping encodings
such as GBK vs CP949.

Example usage:
    >>> import charset_detector
    >>> result = charset_detector.detect(b"Hello, world!")
    >>> print(result.best().encoding)
    UTF-8
"""

from typing import Optional

# Import the Rust extension module
try:
    from charset_detector._detector import (
        CharsetDetector as _CharsetDetector,
        CharsetMatch as _CharsetMatch,
        CharsetMatches as _CharsetMatches,
        detect_from_bytes as _detect_from_bytes,
        detect_best_from_bytes as _detect_best_from_bytes,
        VERSION as _VERSION,
    )
except ImportError:
    # Development mode - module not built yet
    _VERSION = "0.1.0"

__version__ = _VERSION
__all__ = [
    "detect",
    "detect_best",
    "CharsetDetector",
    "CharsetMatch",
    "CharsetMatches",
]

# Re-export with nicer names


def detect(data: bytes, min_confidence: float = 0.3) -> "CharsetMatches":
    """
    Detect charset from bytes.

    Args:
        data: Raw bytes to analyze
        min_confidence: Minimum confidence threshold (0.0 to 1.0)

    Returns:
        CharsetMatches object containing all candidates

    Example:
        >>> results = detect(b"some text")
        >>> best = results.best()
        >>> print(f"Encoding: {best.encoding}, Confidence: {best.confidence}")
    """
    return _detect_from_bytes(data, min_confidence)


def detect_best(data: bytes, min_confidence: float = 0.3) -> "Optional[CharsetMatch]":
    """
    Detect charset and return only the best match.

    Args:
        data: Raw bytes to analyze
        min_confidence: Minimum confidence threshold (0.0 to 1.0)

    Returns:
        CharsetMatch object or None if no match found

    Example:
        >>> match = detect_best(b"some text")
        >>> if match:
        ...     print(f"Encoding: {match.encoding}")
    """
    return _detect_best_from_bytes(data, min_confidence)


class CharsetDetector:
    """
    Charset detector with configurable options.

    Example:
        >>> detector = CharsetDetector(min_confidence=0.5)
        >>> result = detector.detect(b"some text")
    """

    def __init__(self, min_confidence: float = 0.3):
        """
        Create a new detector.

        Args:
            min_confidence: Minimum confidence threshold (0.0 to 1.0)
        """
        self._detector = _CharsetDetector(min_confidence)

    def detect(self, data: bytes) -> "CharsetMatches":
        """
        Detect charset from bytes.

        Args:
            data: Raw bytes to analyze

        Returns:
            CharsetMatches object containing all candidates
        """
        return self._detector.detect(data)

    def detect_best(self, data: bytes) -> "Optional[CharsetMatch]":
        """
        Detect and return only the best match.

        Args:
            data: Raw bytes to analyze

        Returns:
            CharsetMatch object or None if no match found
        """
        return self._detector.detect_best(data)

    def detect_encoding(self, data: bytes) -> Optional[str]:
        """
        Detect and return encoding name only.

        Args:
            data: Raw bytes to analyze

        Returns:
            IANA encoding name or None
        """
        return self._detector.detect_encoding(data)


# Make the Rust classes available
CharsetMatch = _CharsetMatch
CharsetMatches = _CharsetMatches
