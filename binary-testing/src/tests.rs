

#[cfg(test)]
mod test {
	use boxy_cli::prelude::*;
	use std::time::{Duration, Instant};

	fn validate_runtime(time: Duration) -> Result<(), String> {
		if time < Duration::from_millis(100) {
			Ok(())
		}
		else {
			panic!("\n\nRuntime exceeding upper limit!!!\n\n")
		}
	}
	#[test]
	fn bechmark_test() {
		let mut box1 = Boxy::new(BoxType::Bold,"#00ffff");
		box1.add_text_sgmt("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur", "#ffff");
		box1.add_text_sgmt("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.", "#ffff");
		box1.add_text_sgmt("Hello Theree", "#ffff");
		let start = Instant::now();
		box1.display();
		let duration = start.elapsed();
		let mut box2 = boxy!(type: BoxType::Double, color:"#00ffff");
		box2.add_text_sgmt("Hello There Mateojablij trhwesoiuethj 0piswe hjgtgoise jgtowie3thj q3o-oitujpwiej toiq 0iweeh gt owjtpiewrwh WOKWRHJ JRQWE4IHYNE5R bfg oiwhf apeih aepih aepih aepihetm wf[ohgwlMRF [POWQWRF]] [OJTQEA [OJ]]OJBDGISUDBG SIUGRG OGUFOSIJGOSN SOGUIHSGIORNGR ORIRHGOSJRNGOIJRG OPIFGHRPGNPERIJG ORIRGRPIGNERPGOSJH ", "#ffff");
		box2.add_text_sgmt("Hello Theree", "#ffff");
		let start1 = Instant::now();
		box2.display();
		let duration1 = start1.elapsed();
		assert!(validate_runtime(duration).is_ok());
		assert!(validate_runtime(duration1).is_ok());


	}

}
