#[cfg(test)]
mod tests {
    use crate::boxer::text_wrap_vec_fast;
    use crate::prelude::*;

    #[test]
    fn wrap_short_text_single_line() {
        let result = text_wrap_vec_fast("hello", 20, &BoxPad::new());
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("hello"));
    }

    #[test]
    fn wrap_long_text_splits_at_space() {
        let result = text_wrap_vec_fast("hello world", 8, &BoxPad::new());
        assert!(result.len() >= 2);
        assert!(!result[1].starts_with(' '));
    }

    #[test]
    fn wrap_does_not_panic_on_word_longer_than_max() {
        let _ = text_wrap_vec_fast("superlongwordwithnobreaks", 8, &BoxPad::new());
    }

    #[test]
    fn wrap_empty_string_no_panic() {
        let _ = text_wrap_vec_fast("", 20, &BoxPad::new());
    }

    #[test]
    fn wrap_respects_padding_lr() {
        let pad = BoxPad::from_tldr(0, 2, 0, 2);
        let result = text_wrap_vec_fast("a b c d e f g h i j k l m n o p", 20, &pad);
        for line in &result {
            assert!(line.trim().len() <= 16, "line too long: {:?}", line);
        }
    }

    #[test]
    fn wrap_subtracts_two_from_max_len() {
        // text_wrap_vec_fast subtracts 2 from disp_width before wrapping
        // "hello world" = 11 chars, needs disp_width >= 13 to fit on one line
        let fits = text_wrap_vec_fast("hello world", 13, &BoxPad::new());
        assert_eq!(fits.len(), 1);
        let splits = text_wrap_vec_fast("hello world", 11, &BoxPad::new());
        assert!(splits.len() > 1);
    }

    #[test]
    fn wrap_no_leading_space_on_continuation() {
        let result = text_wrap_vec_fast("hello world foo", 10, &BoxPad::new());
        for line in result.iter().skip(1) {
            assert!(
                !line.starts_with(' '),
                "continuation has leading space: {:?}",
                line
            );
        }
    }
}
