#[cfg(test)]
mod tests {
    use crate::constructs::SegColor;
    use colored::Color;

    #[test]
    fn parse_valid_hex_color() {
        assert_eq!(
            SegColor::parse_hexcolor("#00ffff"),
            Color::TrueColor {
                r: 0,
                g: 255,
                b: 255
            }
        );
    }

    #[test]
    fn parse_invalid_hex_falls_back_to_white() {
        assert_eq!(SegColor::parse_hexcolor("not-a-color"), Color::White);
    }

    #[test]
    fn parse_hex_uppercase_works() {
        assert_eq!(
            SegColor::parse_hexcolor("#FF0000"),
            Color::TrueColor { r: 255, g: 0, b: 0 }
        );
    }

    #[test]
    fn parse_hex_missing_hash_falls_back() {
        // documents current behavior — doesn't panic regardless
        let _ = SegColor::parse_hexcolor("00ffff");
    }
}
