mod tests;

use boxy_cli::prelude::*;
use std::time::Instant;
use terminal_size::terminal_size;

fn main() {
    let mut box1 = Boxy::new(BoxType::Bold, "#00ffff");
    println!("Terminal Size: {:?}", terminal_size());
    box1.add_text_sgmt("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur", "#00ffff", BoxAlign::Left);
    box1.add_text_sgmt("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.", "#ffff", BoxAlign::Center);
    box1.add_text_sgmt("Hello Theree", "#00ffff", BoxAlign::Center);
    box1.set_padding(BoxPad::from_tldr(1, 2, 3, 4), BoxPad::uniform(0));
    println!("{:#?}", box1);
    // Post-line insert check
    println!("{:?}", box1);
    println!("\nTEXTBOX:");
    let start = Instant::now();
    box1.display();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    let mut box2 = boxy!(type: BoxType::Double, color:"#00ffff");
    box2.add_text_sgmt("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo ", "#ffff", BoxAlign::Left);
    box2.add_text_line("Hello There Boi", "#32CD32");
    box2.set_padding(BoxPad::from_tldr(1, 2, 3, 4), BoxPad::uniform(7));
    let start1 = Instant::now();
    box2.display();
    let duration1 = start1.elapsed();
    println!("Time elapsed: {:?}", duration1);
    let start2 = Instant::now();
    Boxy::builder()
        .box_type(BoxType::Double)
        .color("#f4f5f3")
        .padding(BoxPad::uniform(20), BoxPad::from_tldr(2, 2, 1, 1))
        .align(BoxAlign::Center)
        .add_segment("Hello, Boxy!", "#ffffff", BoxAlign::Center)
        .add_line("This is a new line.", "#32CD32")
        .add_segment("Another section", "#f19356", BoxAlign::Right)
        .width(50)
        .build()
        .display();
    let duration2 = start2.elapsed();
    println!("Time elapsed: {:?}", duration2);
}
