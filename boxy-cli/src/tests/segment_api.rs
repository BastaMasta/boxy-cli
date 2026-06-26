#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn sect_count_increments_correctly() {
        let mut b = Boxy::new(BoxType::Single, "#ffffff");
        assert_eq!(b.sect_count(), 0);
        b.add_text_sgmt("a", "#ffffff", BoxAlign::Left);
        assert_eq!(b.sect_count(), 1);
        b.add_col_text_sgmt(BoxAlign::Left, 2);
        assert_eq!(b.sect_count(), 2);
        b.add_text_sgmt("b", "#ffffff", BoxAlign::Left);
        assert_eq!(b.sect_count(), 3);
    }

    #[test]
    fn add_col_text_sgmt_seeds_equal_default_ratios() {
        let mut b = Boxy::new(BoxType::Single, "#ffffff");
        b.add_col_text_sgmt(BoxAlign::Left, 4);
        assert_eq!(b.seg_cols_ratio()[0], vec![1, 1, 1, 1]);
    }

    #[test]
    #[should_panic(expected = "ratios were given")]
    fn set_segment_ratios_panics_on_wrong_count() {
        let mut b = Boxy::new(BoxType::Single, "#ffffff");
        b.add_col_text_sgmt(BoxAlign::Left, 3);
        b.set_segment_ratios(0, vec![1, 2]);
    }

    #[test]
    #[should_panic]
    fn set_segment_ratios_panics_on_single_segment() {
        let mut b = Boxy::new(BoxType::Single, "#ffffff");
        b.add_text_sgmt("hello", "#ffffff", BoxAlign::Left);
        b.set_segment_ratios(0, vec![1, 1]);
    }

    #[test]
    #[should_panic]
    fn set_segment_ratios_panics_out_of_bounds() {
        let mut b = Boxy::new(BoxType::Single, "#ffffff");
        b.set_segment_ratios(99, vec![1]);
    }
}
