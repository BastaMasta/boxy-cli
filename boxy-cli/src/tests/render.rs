#[cfg(test)]
mod tests {
    use crate::prelude::*;

    fn strip_ansi(s: &str) -> String {
        let mut out = String::new();
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\x1b' {
                while let Some(&n) = chars.peek() {
                    chars.next();
                    if n == 'm' {
                        break;
                    }
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    fn visible_len(s: &str) -> usize {
        strip_ansi(s).chars().count()
    }

    #[test]
    fn render_returns_nonempty_vec() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_text_sgmt("Hello", "#ffffff", BoxAlign::Left);
        assert!(!b.render(80).is_empty());
    }

    #[test]
    fn render_empty_box_no_panic() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        assert!(b.render(80).len() >= 2);
    }

    #[test]
    fn render_all_lines_same_visible_width() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_text_sgmt("Hello, boxy-cli!", "#ffffff", BoxAlign::Center);
        b.add_text_sgmt("Second segment with longer text", "#ffffff", BoxAlign::Left);
        let lines = b.render(100);
        let first = visible_len(&lines[0]);
        for (i, line) in lines.iter().enumerate() {
            assert_eq!(visible_len(line), first, "line {} width mismatch", i);
        }
    }

    #[test]
    fn render_line_count_matches_structure() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_text_sgmt("Hello", "#ffffff", BoxAlign::Left);
        assert!(b.render(80).len() >= 3);
    }

    #[test]
    fn render_two_segments_has_one_divider() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_text_sgmt("One", "#ffffff", BoxAlign::Left);
        b.add_text_sgmt("Two", "#ffffff", BoxAlign::Left);
        let dividers = b
            .render(80)
            .iter()
            .filter(|l| strip_ansi(l).contains('├'))
            .count();
        assert_eq!(dividers, 1);
    }

    #[test]
    fn render_three_segments_has_two_dividers() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_text_sgmt("One", "#ffffff", BoxAlign::Left);
        b.add_text_sgmt("Two", "#ffffff", BoxAlign::Left);
        b.add_text_sgmt("Three", "#ffffff", BoxAlign::Left);
        let dividers = b
            .render(80)
            .iter()
            .filter(|l| strip_ansi(l).contains('├'))
            .count();
        assert_eq!(dividers, 2);
    }

    #[test]
    fn render_fixed_width_respected() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.set_width(60);
        b.add_text_sgmt("Hello", "#ffffff", BoxAlign::Left);
        for line in b.render(120) {
            assert_eq!(
                visible_len(&line),
                60,
                "line width != 60: {:?}",
                strip_ansi(&line)
            );
        }
    }

    #[test]
    fn render_top_border_correct_chars() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_text_sgmt("Hello", "#ffffff", BoxAlign::Left);
        let lines = b.render(80);
        let top = strip_ansi(&lines[0]);
        assert!(top.contains('┌') && top.contains('┐'));
    }

    #[test]
    fn render_bottom_border_correct_chars() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_text_sgmt("Hello", "#ffffff", BoxAlign::Left);
        let lines = b.render(80);
        let bot = strip_ansi(lines.last().unwrap());
        assert!(bot.contains('└') && bot.contains('┘'));
    }

    #[test]
    fn render_double_border_uses_correct_chars() {
        let mut b = Boxy::new(BoxType::Double, "#00ffff");
        b.add_text_sgmt("Hello", "#ffffff", BoxAlign::Left);
        let lines = b.render(80);
        let top = strip_ansi(&lines[0]);
        let bot = strip_ansi(lines.last().unwrap());
        assert!(top.contains('╔') && top.contains('╗'));
        assert!(bot.contains('╚') && bot.contains('╝'));
    }

    #[test]
    fn render_left_align_text_flush_to_left() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_text_sgmt("Hello", "#ffffff", BoxAlign::Left);
        let lines = b.render(40);
        let plain = strip_ansi(
            lines
                .iter()
                .find(|l| strip_ansi(l).contains("Hello"))
                .unwrap(),
        );
        assert!(
            plain.starts_with("│ Hello"),
            "left flush failed: {:?}",
            plain
        );
    }

    #[test]
    fn render_right_align_text_flush_to_right() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_text_sgmt("Hello", "#ffffff", BoxAlign::Right);
        let lines = b.render(40);
        let plain = strip_ansi(
            lines
                .iter()
                .find(|l| strip_ansi(l).contains("Hello"))
                .unwrap(),
        );
        assert!(
            plain.ends_with("Hello │"),
            "right flush failed: {:?}",
            plain
        );
    }

    #[test]
    fn render_center_align_padding_symmetric() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_text_sgmt("Hi", "#ffffff", BoxAlign::Center);
        let lines = b.render(20);
        let plain = strip_ansi(lines.iter().find(|l| strip_ansi(l).contains("Hi")).unwrap());
        let inner: String = plain
            .chars()
            .skip(1)
            .take(plain.chars().count() - 2)
            .collect();
        let left_spaces = inner.chars().take_while(|c| *c == ' ').count();
        let right_spaces = inner.chars().rev().take_while(|c| *c == ' ').count();
        assert!(
            left_spaces.abs_diff(right_spaces) <= 1,
            "center asymmetric: {} left, {} right in {:?}",
            left_spaces,
            right_spaces,
            inner
        );
    }
}
