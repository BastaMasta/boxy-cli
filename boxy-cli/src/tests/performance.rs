#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::time::{Duration, Instant};

    fn validate_runtime(d: Duration) {
        assert!(
            d < Duration::from_millis(100),
            "render exceeded 100ms: {:?}",
            d
        );
    }

    #[test]
    fn benchmark_render_simple() {
        let mut b = Boxy::new(BoxType::Bold, "#00ffff");
        b.add_text_sgmt("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", "#ffffff", BoxAlign::Left);
        b.add_text_sgmt("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium totam rem aperiam", "#ffffff", BoxAlign::Center);
        b.add_text_sgmt("Hello There", "#ffffff", BoxAlign::Right);
        validate_runtime(Instant::now().elapsed());
        let start = Instant::now();
        let _ = b.render(120);
        validate_runtime(start.elapsed());
    }

    #[test]
    fn benchmark_render_columnar() {
        let mut b = Boxy::new(BoxType::Single, "#ff00ff");
        b.add_col_text_sgmt(BoxAlign::Left, 3);
        b.add_col_text_line_indx("Column A line 1", "#ff0000", &0, &0);
        b.add_col_text_line("Column B line 1", "#00ff00", &1);
        b.add_col_text_line("Column C line 1", "#0000ff", &2);
        b.set_segment_ratios(0, vec![1, 2, 1]);
        let start = Instant::now();
        let _ = b.render(120);
        validate_runtime(start.elapsed());
    }

    #[test]
    fn benchmark_render_heavy() {
        let mut b = Boxy::new(BoxType::Double, "#ff00ff");
        b.set_int_padding(BoxPad::vh(1, 2));
        b.set_ext_padding(BoxPad::uniform(1));
        for i in 0..5 {
            b.add_text_sgmt(
                &format!("Segment {} — Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt", i),
                "#ffffff", BoxAlign::Left,
            );
        }
        let start = Instant::now();
        let _ = b.render(120);
        validate_runtime(start.elapsed());
    }
}
