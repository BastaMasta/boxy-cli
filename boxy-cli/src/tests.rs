#[cfg(test)]
mod tests {
    use crate::*;
    use std::time::{Duration, Instant};

    fn validate_runtime(time: Duration) -> Result<(), String> {
        if time < Duration::from_millis(100) {
            Ok(())
        } else {
            panic!("\n\nRuntime exceeding upper limit!!!\n\n")
        }
    }

    #[test]
    fn bechmark_test() {
        let mut box1 = Boxy::new(BoxType::Bold, "#00ffff");
        box1.add_text_sgmt("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur", "#ffff", BoxAlign::Left);
        box1.add_text_sgmt("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.", "#ffff", BoxAlign::Center);
        box1.add_text_sgmt("Hello Theree", "#ffff", BoxAlign::Right);
        let start = Instant::now();
        box1.display();
        let duration = start.elapsed();
        let mut box2 = boxy!(type: BoxType::Double, color:"#00ffff");
        box2.add_text_sgmt("Hello There Mateojablij trhwesoiuethj 0piswe hjgtgoise jgtowie3thj q3o-oitujpwiej toiq 0iweeh gt owjtpiewrwh WOKWRHJ JRQWE4IHYNE5R bfg oiwhf apeih aepih aepih aepihetm wf[ohgwlMRF [POWQWRF]] [OJTQEA [OJ]]OJBDGISUDBG SIUGRG OGUFOSIJGOSN SOGUIHSGIORNGR ORIRHGOSJRNGOIJRG OPIFGHRPGNPERIJG ORIRGRPIGNERPGOSJH ", "#ffff", BoxAlign::Center);
        box2.add_text_sgmt("Hello Theree", "#ffff", BoxAlign::Right);
        let start1 = Instant::now();
        box2.display();
        let duration1 = start1.elapsed();
        assert!(validate_runtime(duration).is_ok());
        assert!(validate_runtime(duration1).is_ok());
    }

    #[test]
    fn columnar_api_test() {
        // exercises add_col_text_sgmt, add_col_text_line_indx, add_col_text_line,
        // and set_segment_ratios together, across more than 2 columns (regression
        // coverage for the column-count/ratio-length mismatch bug)
        let mut box1 = Boxy::new(BoxType::Single, "#ff00ff");
        box1.add_col_text_sgmt(BoxAlign::Left, 3);
        box1.add_col_text_line_indx("Column A line 1", "#ff0000", &0usize, &0usize);
        box1.add_col_text_line("Column B line 1", "#00ff00", &1usize);
        box1.add_col_text_line("Column C line 1", "#0000ff", &2usize);
        box1.set_segment_ratios(0, vec![1, 2, 1]);

        let start = Instant::now();
        box1.display();
        let duration = start.elapsed();
        assert!(validate_runtime(duration).is_ok());
    }

    #[test]
    #[should_panic(expected = "column_count must be at least 1")]
    fn zero_column_segment_panics() {
        let mut box1 = Boxy::new(BoxType::Single, "#ff00ff");
        box1.add_col_text_sgmt(BoxAlign::Left, 0);
    }

    #[test]
    #[should_panic(expected = "ratios were given")]
    fn mismatched_ratio_length_panics() {
        let mut box1 = Boxy::new(BoxType::Single, "#ff00ff");
        box1.add_col_text_sgmt(BoxAlign::Left, 3);
        box1.set_segment_ratios(0, vec![1, 2]); // only 2 ratios for 3 columns
    }
}
