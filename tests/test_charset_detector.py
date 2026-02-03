"""
Python unit tests for charset_detector library
"""

import pytest
import charset_detector
from charset_detector import CharsetDetector, CharsetMatch, CharsetMatches


class TestDetectFunction:
    """测试 detect 函数"""

    def test_detect_utf8_ascii(self):
        """测试检测 ASCII 文本 (UTF-8)"""
        result = charset_detector.detect(b"Hello, World!")
        assert result is not None
        assert result.best is not None
        assert result.best.encoding == "UTF-8"
        assert result.best.confidence > 0.9

    def test_detect_utf8_chinese(self):
        """测试检测 UTF-8 中文"""
        # "世界" in UTF-8
        data = b"Hello \xe4\xb8\x96\xe7\x95\x8c!"
        result = charset_detector.detect(data)
        assert result.best.encoding == "UTF-8"
        assert result.best.confidence > 0.9

    def test_detect_gbk(self):
        """测试检测 GBK 编码"""
        # "你好世界" in GBK
        gbk_bytes = bytes([0xC4, 0xE3, 0xBA, 0xC3, 0xCA, 0xC0, 0xBD, 0xE7])
        result = charset_detector.detect(gbk_bytes)
        assert result.best.encoding == "GBK"
        assert result.best.confidence > 0.8
        assert result.best.language == "zh"

    def test_detect_cp949(self):
        """测试检测 CP949 (韩文) 编码"""
        # "안녕하세요" in CP949
        cp949_bytes = bytes([0xBE, 0xC8, 0xB3, 0xE7, 0xC7, 0xCF, 0xBC, 0xBC, 0xBF, 0xE4])
        result = charset_detector.detect(cp949_bytes)
        assert result.best.encoding == "CP949"
        assert result.best.confidence > 0.8
        assert result.best.language == "ko"

    def test_detect_gbk_vs_cp949_discrimination(self):
        """测试 GBK vs CP949 区分 (0x40 尾字节规则)"""
        # 包含 0x40 作为尾字节的序列（GBK 特有）
        gbk_specific = bytes([0x81, 0x40, 0xD2, 0xBB])
        result = charset_detector.detect(gbk_specific)
        assert result.best.encoding == "GBK"
        assert result.best.confidence > 0.9  # 硬规则应该有高置信度

    def test_detect_empty_data(self):
        """测试空数据"""
        result = charset_detector.detect(b"")
        assert result is not None
        # 空数据应该返回某种默认编码
        assert result.best is not None


class TestDetectBestFunction:
    """测试 detect_best 快捷函数"""

    def test_detect_best_utf8(self):
        """测试 detect_best 函数 - UTF-8"""
        best = charset_detector.detect_best(b"Hello")
        assert best is not None
        assert best.encoding == "UTF-8"

    def test_detect_best_gbk(self):
        """测试 detect_best 函数 - GBK"""
        gbk_bytes = bytes([0xC4, 0xE3, 0xBA, 0xC3])
        best = charset_detector.detect_best(gbk_bytes)
        assert best is not None
        assert best.encoding == "GBK"

    def test_detect_best_returns_type(self):
        """测试 detect_best 返回类型"""
        best = charset_detector.detect_best(b"test")
        assert isinstance(best, CharsetMatch)
        assert hasattr(best, 'encoding')
        assert hasattr(best, 'confidence')
        assert hasattr(best, 'language')
        assert hasattr(best, 'method')


class TestCharsetDetector:
    """测试 CharsetDetector 类"""

    def test_detector_default_confidence(self):
        """测试默认置信度阈值"""
        detector = CharsetDetector()
        result = detector.detect(b"Hello")
        assert result.best is not None

    def test_detector_custom_confidence(self):
        """测试自定义置信度阈值"""
        detector = CharsetDetector(min_confidence=0.95)
        result = detector.detect(b"Hello")
        # 高置信度阈值可能返回更少的结果

    def test_detector_detect_method(self):
        """测试 detect 方法"""
        detector = CharsetDetector()
        result = detector.detect(b"Hello")
        assert isinstance(result, CharsetMatches)

    def test_detector_detect_best_method(self):
        """测试 detect_best 方法"""
        detector = CharsetDetector()
        best = detector.detect_best(b"Hello")
        assert isinstance(best, CharsetMatch)

    def test_detector_detect_encoding_method(self):
        """测试 detect_encoding 方法"""
        detector = CharsetDetector()
        encoding = detector.detect_encoding(b"Hello")
        assert encoding == "UTF-8"

    def test_detector_detect_encoding_none_case(self):
        """测试 detect_encoding 对无法识别的数据"""
        detector = CharsetDetector()
        encoding = detector.detect_encoding(b"")
        # 空数据可能返回 None 或默认编码


class TestCharsetMatch:
    """测试 CharsetMatch 类"""

    def test_match_attributes(self):
        """测试 CharsetMatch 属性"""
        best = charset_detector.detect_best(b"Hello")
        assert hasattr(best, 'encoding')
        assert hasattr(best, 'confidence')
        assert hasattr(best, 'language')
        assert hasattr(best, 'method')
        assert isinstance(best.encoding, str)
        assert isinstance(best.confidence, float)
        assert 0.0 <= best.confidence <= 1.0

    def test_match_repr(self):
        """测试 CharsetMatch 字符串表示"""
        best = charset_detector.detect_best(b"Hello")
        repr_str = repr(best)
        assert "CharsetMatch" in repr_str
        assert "encoding" in repr_str

    def test_match_str(self):
        """测试 CharsetMatch __str__"""
        best = charset_detector.detect_best(b"Hello")
        str_str = str(best)
        assert len(str_str) > 0


class TestCharsetMatches:
    """测试 CharsetMatches 类"""

    def test_matches_best_property(self):
        """测试 best 属性"""
        result = charset_detector.detect(b"Hello")
        assert isinstance(result.best, CharsetMatch)

    def test_matches_all_method(self):
        """测试 all 方法"""
        result = charset_detector.detect(b"Hello")
        all_matches = result.all()
        assert isinstance(all_matches, list)
        assert len(all_matches) >= 1

    def test_matches_len(self):
        """测试 __len__"""
        result = charset_detector.detect(b"Hello")
        assert len(result) >= 1

    def test_matches_iteration(self):
        """测试迭代"""
        result = charset_detector.detect(b"Hello")
        count = 0
        for match in result:
            assert isinstance(match, CharsetMatch)
            count += 1
            if count >= 3:  # 只测试前几个
                break

    def test_matches_repr(self):
        """测试 CharsetMatches 字符串表示"""
        result = charset_detector.detect(b"Hello")
        repr_str = repr(result)
        assert "CharsetMatches" in repr_str


class TestGBKCp949Discrimination:
    """GBK vs CP949 区分专项测试"""

    def test_chinese_common_chars(self):
        """测试中文常用字符"""
        # 包含多个中文常用字符
        chinese_text = bytes([0xB5, 0xC4, 0xCA, 0xC7, 0xD2, 0xBB])  # 的、是、一
        result = charset_detector.detect(chinese_text)
        assert result.best.encoding == "GBK"
        assert result.best.language == "zh"

    def test_korean_common_chars(self):
        """测试韩文常用字符"""
        # 包含韩文常用字符
        korean_text = bytes([0xBE, 0xC8, 0xB3, 0xE7, 0xC7, 0xCF])  # 안녕하
        result = charset_detector.detect(korean_text)
        assert result.best.encoding == "CP949"
        assert result.best.language == "ko"

    def test_gbk_hard_rule_0x40(self):
        """测试 GBK 硬规则：0x40 尾字节"""
        # CP949 不允许 0x40 作为尾字节
        gkb_only = bytes([0x81, 0x40])
        result = charset_detector.detect(gkb_only)
        assert result.best.encoding == "GBK"
        assert result.best.confidence > 0.9  # 硬规则应该有高置信度

    def test_mixed_chinese_korean_bias(self):
        """测试混合文本的倾向性"""
        # "中文" in GBK
        chinese = bytes([0xD6, 0xD0, 0xCE, 0xC4])
        result = charset_detector.detect(chinese)
        # 应该识别为中文
        assert result.best.language == "zh"


class TestEdgeCases:
    """边界情况测试"""

    def test_single_byte(self):
        """测试单字节数据"""
        result = charset_detector.detect(b"H")
        assert result.best is not None

    def test_very_long_data(self):
        """测试长数据"""
        long_data = b"Hello, World! " * 1000
        result = charset_detector.detect(long_data)
        assert result.best is not None

    def test_all_zeros(self):
        """测试全零数据"""
        result = charset_detector.detect(b"\x00" * 100)
        assert result.best is not None

    def test_high_bytes(self):
        """测试高字节值"""
        result = charset_detector.detect(b"\xFF\xFE\xFD" * 10)
        assert result.best is not None

    def test_null_bytes(self):
        """测试包含 null 字节"""
        result = charset_detector.detect(b"Hello\x00World")
        assert result.best is not None


class TestConfidenceLevels:
    """置信度测试"""

    def test_utf8_bom_high_confidence(self):
        """测试 UTF-8 BOM 的高置信度"""
        bom_data = b"\xEF\xBB\xBFHello"
        result = charset_detector.detect(bom_data)
        assert result.best.encoding == "UTF-8"
        assert result.best.confidence > 0.95

    def test_valid_utf8_sequence(self):
        """测试有效的 UTF-8 序列"""
        # 有效的 3 字节 UTF-8 序列
        utf8_valid = b"\xe4\xb8\x96\xe7\x95\x8c"  # "世界"
        result = charset_detector.detect(utf8_valid)
        assert result.best.encoding == "UTF-8"
        assert result.best.confidence > 0.9

    def test_low_confidence_case(self):
        """测试低置信度情况"""
        # 简单的 ASCII 可能被多种编码匹配
        result = charset_detector.detect(b"ABC")
        # 置信度可能不是特别高，因为很多编码都支持 ASCII
        assert result.best.confidence > 0.0


class TestLanguageDetection:
    """语言检测测试"""

    def test_chinese_language(self):
        """测试中文语言检测"""
        gbk_bytes = bytes([0xC4, 0xE3, 0xBA, 0xC3])  # "你好"
        result = charset_detector.detect(gbk_bytes)
        assert result.best.language == "zh"

    def test_korean_language(self):
        """测试韩文语言检测"""
        cp949_bytes = bytes([0xBE, 0xC8, 0xB3, 0xE7])  # "안녕"
        result = charset_detector.detect(cp949_bytes)
        assert result.best.language == "ko"

    def test_english_no_language(self):
        """测试英文（可能不返回语言）"""
        result = charset_detector.detect(b"Hello World")
        # 英文可能不检测特定语言
        # language 可能是 None 或 "en" 或 "und"
        assert result.best is not None


class TestMethodDetection:
    """检测方法测试"""

    def test_bom_method(self):
        """测试 BOM 检测方法"""
        bom_data = b"\xEF\xBB\xBFHello"
        result = charset_detector.detect(bom_data)
        assert result.best.method == "bom"

    def test_utf8_validation_method(self):
        """测试 UTF-8 验证方法"""
        result = charset_detector.detect(b"Hello")
        # 应该使用某种验证方法
        assert result.best.method is not None
        assert isinstance(result.best.method, str)


@pytest.mark.parametrize("data,expected_encoding", [
    (b"Hello", "UTF-8"),
    (bytes([0xC4, 0xE3, 0xBA, 0xC3]), "GBK"),  # 你好
    (bytes([0xBE, 0xC8, 0xB3, 0xE7]), "CP949"),  # 안녕
])
def test_paramized_detection(data, expected_encoding):
    """参数化检测测试"""
    result = charset_detector.detect(data)
    assert result.best.encoding == expected_encoding


class TestVersion:
    """版本信息测试"""

    def test_version_exists(self):
        """测试版本信息存在"""
        assert hasattr(charset_detector, '__version__')
        assert isinstance(charset_detector.__version__, str)

    def test_version_format(self):
        """测试版本格式"""
        version = charset_detector.__version__
        parts = version.split('.')
        assert len(parts) >= 2
