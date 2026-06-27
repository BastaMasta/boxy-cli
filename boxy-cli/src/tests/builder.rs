#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn builder_produces_correct_sect_count() {
        let b = Boxy::builder()
            .add_segment("a", "#ffffff", BoxAlign::Left)
            .add_segment("b", "#ffffff", BoxAlign::Left)
            .build();
        assert_eq!(b.sect_count(), 2);
    }

    #[test]
    fn builder_segment_ratios_applied() {
        let b = Boxy::builder()
            .add_col_segment(BoxAlign::Left, 3)
            .segment_ratios(0, vec![1, 2, 1])
            .build();
        assert_eq!(b.seg_cols_ratio()[0], vec![1, 2, 1]);
    }

    #[test]
    fn builder_render_no_panic() {
        let mut b = Boxy::builder()
            .box_type(BoxType::Double)
            .color("#00ffff")
            .add_segment("Hello", "#ffffff", BoxAlign::Center)
            .add_line("Second line", "#aaaaaa")
            .add_segment("Another section", "#f19356", BoxAlign::Right)
            .build();
        assert!(!b.render(100).is_empty());
    }

    #[test]
    fn builder_empty_build_no_panic() {
        let mut b = Boxy::builder().build();
        let _ = b.render(80);
    }
}
