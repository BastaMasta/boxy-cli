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
    fn render_columnar_no_panic() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_col_text_sgmt(BoxAlign::Left, 3);
        b.add_col_text_line("A", "#ffffff", &0);
        b.add_col_text_line("B", "#ffffff", &1);
        b.add_col_text_line("C", "#ffffff", &2);
        assert!(!b.render(100).is_empty());
    }

    #[test]
    fn render_columnar_lines_consistent_width() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_col_text_sgmt(BoxAlign::Left, 2);
        b.add_col_text_line("Left column", "#ffffff", &0);
        b.add_col_text_line("Right column", "#ffffff", &1);
        let lines = b.render(100);
        let first = visible_len(&lines[0]);
        for (i, line) in lines.iter().enumerate() {
            assert_eq!(
                visible_len(line),
                first,
                "columnar line {} width mismatch",
                i
            );
        }
    }

    #[test]
    fn render_columnar_divider_has_upper_t() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_text_sgmt("Header", "#ffffff", BoxAlign::Center);
        b.add_col_text_sgmt(BoxAlign::Left, 3);
        b.add_col_text_line("A", "#ffffff", &0);
        b.add_col_text_line("B", "#ffffff", &1);
        b.add_col_text_line("C", "#ffffff", &2);
        assert!(b.render(120).iter().any(|l| strip_ansi(l).contains('┬')));
    }

    #[test]
    fn render_columnar_bottom_has_lower_t() {
        let mut b = Boxy::new(BoxType::Single, "#00ffff");
        b.add_col_text_sgmt(BoxAlign::Left, 2);
        b.add_col_text_line("A", "#ffffff", &0);
        b.add_col_text_line("B", "#ffffff", &1);
        let lines = b.render(100);
        assert!(strip_ansi(lines.last().unwrap()).contains('┴'));
    }

    #[test]
    fn col_widths_sum_equals_printable() {
        let mut b = Boxy::new(BoxType::Single, "#ffffff");
        b.add_col_text_sgmt(BoxAlign::Left, 3);
        b.set_segment_ratios(0, vec![1, 1, 1]);
        for disp in [60usize, 80, 100, 120, 200] {
            let widths = b.col_widths(&0, &disp);
            assert_eq!(
                widths.iter().sum::<usize>(),
                disp.saturating_sub(2),
                "widths don't sum to printable for disp={}",
                disp
            );
        }
    }

    #[test]
    fn col_widths_single_column_full_width() {
        let mut b = Boxy::new(BoxType::Single, "#ffffff");
        b.add_col_text_sgmt(BoxAlign::Left, 1);
        assert_eq!(b.col_widths(&0, &80), vec![80]);
    }

    #[test]
    fn col_widths_unequal_ratios_proportional() {
        let mut b = Boxy::new(BoxType::Single, "#ffffff");
        b.add_col_text_sgmt(BoxAlign::Left, 2);
        b.set_segment_ratios(0, vec![1, 3]);
        let widths = b.col_widths(&0, &100);
        assert_eq!(widths.iter().sum::<usize>(), 99);
        assert_eq!(widths[0], 24);
        assert_eq!(widths[1], 75);
    }

    #[test]
    fn col_widths_last_col_gets_remainder() {
        let mut b = Boxy::new(BoxType::Single, "#ffffff");
        b.add_col_text_sgmt(BoxAlign::Left, 3);
        b.set_segment_ratios(0, vec![1, 1, 1]);
        let widths = b.col_widths(&0, &101);
        assert_eq!(widths, vec![33, 33, 33]);
    }

    #[test]
    fn col_boundaries_empty_returns_empty() {
        let b = Boxy::new(BoxType::Single, "#ffffff");
        assert_eq!(b.col_boundaries(&[]), vec![]);
    }

    #[test]
    fn col_boundaries_single_col_returns_empty() {
        let b = Boxy::new(BoxType::Single, "#ffffff");
        assert_eq!(b.col_boundaries(&[50]), vec![]);
    }

    #[test]
    fn col_boundaries_two_cols_correct() {
        let b = Boxy::new(BoxType::Single, "#ffffff");
        assert_eq!(b.col_boundaries(&[30, 70]), vec![30]);
    }

    #[test]
    fn col_boundaries_three_cols_correct() {
        let b = Boxy::new(BoxType::Single, "#ffffff");
        assert_eq!(b.col_boundaries(&[25, 25, 50]), vec![25, 51]);
    }

    #[test]
    fn col_boundaries_count_is_col_count_minus_one() {
        let b = Boxy::new(BoxType::Single, "#ffffff");
        for n in 2..=6 {
            assert_eq!(b.col_boundaries(&vec![20usize; n]).len(), n - 1);
        }
    }

    #[test]
    #[should_panic(expected = "column_count must be at least 1")]
    fn add_col_text_sgmt_panics_on_zero_columns() {
        let mut b = Boxy::new(BoxType::Single, "#ff00ff");
        b.add_col_text_sgmt(BoxAlign::Left, 0);
    }

    #[test]
    #[should_panic]
    fn add_col_text_line_panics_on_invalid_col_index() {
        let mut b = Boxy::new(BoxType::Single, "#ffffff");
        b.add_col_text_sgmt(BoxAlign::Left, 2);
        b.add_col_text_line("text", "#ffffff", &2);
    }
}
