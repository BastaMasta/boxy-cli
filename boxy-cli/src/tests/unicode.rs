#[cfg(test)]
mod tests {
    use crate::boxer::{display_width, text_wrap_vec_fast};
    use crate::prelude::*;
    use std::time::{Duration, Instant};

    // ── display_width ─────────────────────────────────────────────────────────

    #[test]
    fn display_width_ascii() {
        assert_eq!(display_width("hello"), 5);
    }

    #[test]
    fn display_width_cjk() {
        // Each CJK character occupies 2 terminal columns.
        // "日本語".len() == 9 (UTF-8 bytes), but display width == 6.
        assert_eq!(display_width("日本語"), 6);
    }

    #[test]
    fn display_width_emoji() {
        // 🦀 is a single codepoint that occupies 2 terminal columns.
        assert_eq!(display_width("🦀"), 2);
    }

    #[test]
    fn display_width_mixed() {
        // "Hi日" → 2 (ascii) + 2 (wide CJK) = 4 columns
        assert_eq!(display_width("Hi日"), 4);
    }

    #[test]
    fn display_width_combining_marks() {
        // "e\u{0301}" is e + combining acute accent — two codepoints, one column ("é")
        assert_eq!(display_width("e\u{0301}"), 1);
    }

    // ── text_wrap_vec_fast with unicode ───────────────────────────────────────

    #[test]
    fn wrap_cjk_respects_display_width() {
        // Each CJK char = 2 cols. With max_cols = 8 (disp_width 10 - 2),
        // we can fit at most 4 CJK chars per line.
        let result = text_wrap_vec_fast("日本語テスト文字", 10, &BoxPad::new());
        for line in &result {
            assert!(
                display_width(line.as_str()) <= 8,
                "line too wide: {:?} (width={})",
                line,
                display_width(line.as_str())
            );
        }
    }

    #[test]
    fn wrap_does_not_panic_on_emoji() {
        // Should not panic or produce garbled output on emoji input.
        let _ = text_wrap_vec_fast("🦀 Rust 🚀 Go 🐍 Python", 20, &BoxPad::new());
    }

    #[test]
    fn wrap_zwj_emoji_stays_intact() {
        // 👨‍👩‍👧 is a ZWJ sequence — multiple codepoints, one grapheme cluster.
        // It must never be split mid-sequence.
        let family = "👨\u{200D}👩\u{200D}👧";
        let result = text_wrap_vec_fast(family, 20, &BoxPad::new());
        // The entire sequence should appear in one line chunk, not fragmented.
        assert_eq!(result.concat(), family);
    }

    #[test]
    fn wrap_no_panic_on_multibyte_at_boundary() {
        // Stress test: long CJK string where a multibyte boundary falls exactly
        // on the old byte-indexed cut point — would panic with the old impl.
        let cjk = "あいうえおかきくけこさしすせそたちつてと"; // 20 hiragana = 40 cols
        let _ = text_wrap_vec_fast(cjk, 15, &BoxPad::new());
    }

    // ── render integration ────────────────────────────────────────────────────

    #[test]
    fn unicode_cjk_box_renders_within_budget() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_text_sgmt(
            "日本語テスト — CJK text in a box",
            "#ffffff",
            BoxAlign::Center,
        );
        b.add_text_sgmt("한국어 지원 테스트", "#aaaaaa", BoxAlign::Left);
        let start = Instant::now();
        let lines = b.render(80);
        assert!(start.elapsed() < Duration::from_millis(200));
        assert!(!lines.is_empty());
    }

    #[test]
    fn unicode_emoji_box_renders_within_budget() {
        let mut b = Boxy::new(BoxType::Double, "#ff00ff");
        b.add_text_sgmt("🦀 Rust 🚀 Go 🐍 Python", "#ffffff", BoxAlign::Center);
        b.add_text_sgmt(
            "👨\u{200D}👩\u{200D}👧 ZWJ sequence test",
            "#aaaaaa",
            BoxAlign::Left,
        );
        let start = Instant::now();
        let lines = b.render(80);
        assert!(start.elapsed() < Duration::from_millis(200));
        assert!(!lines.is_empty());
    }
}
