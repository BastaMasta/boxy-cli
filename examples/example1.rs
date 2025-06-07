use boxy_cli::prelude::*;
use boxy_cli::constructs::Boxy;

fn main() {
    // Create a new Boxy instance
    Boxy::builder()
        .box_type(BoxType::Double)
        .color("#00ffff")
        .padding(BoxPad::uniform(1), BoxPad::from_tldr(2, 2, 1, 1))
        .align(BoxAlign::Center)
        .add_segment("Hello, Boxy!", "#ffffff")
        .add_line("This is a new line.", "#32CD32")
        .add_segment("Another section", "#663399")
        .width(50)
        .build()
        .display();
}
