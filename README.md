# charset-detector

High-performance character encoding detection library written in Rust with Python bindings.

## Features

- **Fast**: Rust implementation provides 5-10x speedup over pure Python solutions
- **Accurate**: Multi-factor detection strategy handles encoding overlaps (GBK vs CP949)
- **Comprehensive**: Support for UTF-8, UTF-16, UTF-32, GBK, GB18030, Big5, Shift_JIS, CP949, EUC-JP, EUC-KR, ISO-2022 variants, and many single-byte encodings
- **Python API**: Easy-to-use Python API compatible with charset_normalizer

## Installation

```bash
pip install charset-detector
```

## Quick Start

### Python

```python
import charset_detector

# Simple detection
result = charset_detector.detect(b"Hello, world!")
print(f"Encoding: {result.best.encoding}")
print(f"Confidence: {result.best.confidence}")

# Get only the best match
best = charset_detector.detect_best(b"some text")
if best:
    print(f"Detected: {best.encoding}")

# Advanced: custom detector
detector = charset_detector.CharsetDetector(min_confidence=0.5)
result = detector.detect(b"some text")
```

### Rust

```rust
use charset_detector::{detect, detect_best};

// Detect all candidates
let results = detect(b"Hello, world!");
if let Some(best) = detect_best(b"Hello, world!") {
    println!("Detected: {} with {:.2}% confidence",
             best.encoding, best.confidence * 100.0);
}
```

## Supported Encodings

### Unicode
- UTF-8, UTF-16 LE/BE, UTF-32 LE/BE

### CJK (Chinese, Japanese, Korean)
- GBK, GB18030 (Chinese)
- Big5 (Traditional Chinese)
- Shift_JIS, EUC-JP (Japanese)
- CP949, EUC-KR (Korean)
- ISO-2022-JP, ISO-2022-KR

### Single-byte encodings
- ISO-8859-1 through ISO-8859-16
- Windows-1250 through Windows-1258
- MacRoman, IBM437, IBM850

## GBK vs CP949 Discrimination

This library uses a multi-factor approach to distinguish between GBK and CP949,
which have significant byte sequence overlap:

1. **Hard Rule**: CP949 excludes 0x40 as trail byte (valid in GBK)
2. **Character Frequency**: Chinese vs Korean character frequency analysis
3. **Code Point Ranges**: GBK/CP949-specific code point regions

## Performance

Benchmark results (1MB of text):

| Encoding | Time | Comparison |
|----------|------|------------|
| UTF-8 | ~2ms | 10x faster than Python |
| GBK | ~8ms | 5x faster than Python |
| CP949 | ~8ms | 5x faster than Python |

## Development

```bash
# Build the Rust library
cargo build

# Build Python wheel
maturin develop

# Run tests
cargo test
pytest tests/

# Run benchmarks
cargo bench
```

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
