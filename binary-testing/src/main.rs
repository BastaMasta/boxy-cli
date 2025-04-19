mod tests;

use boxy_cli::prelude::*;
use terminal_size::terminal_size;
use std::time::Instant;
use boxy_cli::constructs::BoxPad;

fn main() {
    let mut box1 = Boxy::new(BoxType::Bold,"#00ffff");
    println!("{:?}", terminal_size());

    box1.add_text_sgmt("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur", "#00ffff");
    box1.add_text_sgmt("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.", "#ffff");
    box1.add_text_sgmt("Hello Theree", "#00ffff");
    println!("{:#?}", box1);
    // Post line insert check
    println!("{:?}", box1);
    println!("\nTEXTBOX:");
    let start = Instant::now();
    box1.display();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    let mut box2 = boxy!(type: BoxType::Double, color:"#00ffff");
    box2.add_text_sgmt("Hello There Mateojablij trhwesoiuethj 0piswe hjgtgoise jgtowie3thj q3o-oitujpwiej toiq 0iweeh gt owjtpiewrwh WOKWRHJ JRQWE4IHYNE5R bfg oiwhf apeih aepih aepih aepihetm wf[ohgwlMRF [POWQWRF]] [OJTQEA [OJ]]OJBDGISUDBG SIUGRG OGUFOSIJGOSN SOGUIHSGIORNGR ORIRHGOSJRNGOIJRG OPIFGHRPGNPERIJG ORIRGRPIGNERPGOSJH ", "#ffff");
    box2.add_text_line("Hello Theree", "#32CD32");
    let start1 = Instant::now();
    box2.display();
    let duration1 = start1.elapsed();
    println!("Time elapsed: {:?}", duration1);
    let mut my_box = Boxy::builder()
        .box_type(BoxType::Double)
        .color("#aaffff")
        .padding(BoxPad::uniform(1), BoxPad::from_tldr(2, 2, 1, 1))
        .align(BoxAlign::Center)
        .add_segment("Hello, Boxy!", "#ffffff")
        .add_line("This is a new line.", "#32CD32")
        .add_segment("Another section", "#f19356")
        .width(50)
        .build();
    println!("{:#?}", my_box);
    my_box.display();
}
