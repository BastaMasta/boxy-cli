mod tests;

use boxy_cli::prelude::*;
use terminal_size::terminal_size;
use std::time::Instant;


fn main() {
    let mut box1 = Boxy::new(BoxType::Bold,"#00ffff");
    // Post line insert check
    println!("{:#?}", box1);
    println!("{:?}", terminal_size());

    box1.add_text_sgmt("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur", "#ffff");
    box1.add_text_sgmt("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.", "#ffff");
    box1.add_text_sgmt("Hello Theree", "#ffff");
    // Post line insert check
    println!("{:?}", box1);
    println!("\nTEXTBOX:");
    let start = Instant::now();
    box1.display();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    let mut box2 = boxy!(type: BoxType::Double, color:"#00ffff");
    box2.add_text_sgmt("Hello There Mateojablij trhwesoiuethj 0piswe hjgtgoise jgtowie3thj q3o-oitujpwiej toiq 0iweeh gt owjtpiewrwh WOKWRHJ JRQWE4IHYNE5R bfg oiwhf apeih aepih aepih aepihetm wf[ohgwlMRF [POWQWRF]] [OJTQEA [OJ]]OJBDGISUDBG SIUGRG OGUFOSIJGOSN SOGUIHSGIORNGR ORIRHGOSJRNGOIJRG OPIFGHRPGNPERIJG ORIRGRPIGNERPGOSJH ", "#ffff");
    box2.add_text_sgmt("Hello Theree", "#ffff");
    let start1 = Instant::now();
    box2.display();
    let duration1 = start1.elapsed();
    println!("Time elapsed: {:?}", duration1);
    
    let mut boxy = Boxy::default();
    boxy.add_text_sgmt("This is the first line of text.", "#ff0000");
    boxy.add_text_line("This is the second line of text.");
    boxy.add_text_line("This is the third line of text.");
    
    // Set ratios for the first segment
    boxy.set_segment_ratios(0, vec![1, 3, 2]);
    
    // Display the first segment divided into mini-segments
    boxy.display_segment_with_ratios(0, &80);

}
