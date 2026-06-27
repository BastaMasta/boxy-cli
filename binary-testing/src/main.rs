mod tests;

use boxy_cli::prelude::*;
use std::time::Instant;
use terminal_size::terminal_size;

fn main() {
    let mut box1 = Boxy::new(BoxType::Bold, "#00ffff");
    println!("Terminal Size: {:?}", terminal_size());
    box1.add_text_sgmt("😀 😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀😀 ááááááááááááááááááááááááááááááááááááááááááááááá Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur", "#00ffff", BoxAlign::Left);
    box1.add_text_sgmt("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.", "#ffff", BoxAlign::Center);
    box1.add_text_sgmt("Hello Theree", "#00ffff", BoxAlign::Center);
    box1.set_padding(BoxPad::from_tldr(1, 2, 3, 7), BoxPad::uniform(3));
    // println!("{:?}", box1);
    println!("\nTEXTBOX:");
    let start = Instant::now();
    box1.display();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    let mut box2 = boxy!(type: BoxType::Double, color:"#00ffff");
    box2.add_text_sgmt("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo ", "#ffff", BoxAlign::Left);
    box2.add_text_line("Hello There Boi", "#32CD32");
    box2.set_padding(BoxPad::from_tldr(1, 2, 3, 4), BoxPad::uniform(7));
    box2.add_col_text_sgmt(BoxAlign::Center, 4);
    box2.add_col_text_line_indx("datastring1", "#32CD32", &1usize, &0usize);
    box2.add_col_text_line("col1 row1", "#ff5555", &0usize);
    box2.add_col_text_line("col2 row1", "#55ff55", &1usize);
    box2.add_col_text_line("col3 row1", "#5555ff", &2usize);
    box2.add_col_text_line("col4 row1", "#ffff55", &3usize);
    box2.set_segment_ratios(1, vec![1, 2, 3, 4]);
    box2.set_align(BoxAlign::Center);
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
    Boxy::builder()
        .add_segment("Status Report", "#ffffff", BoxAlign::Center)
        .add_col_segment(BoxAlign::Left, 3)
        .add_col_line("Name", "#ffffff", 0)
        .add_col_line("Status", "#ffffff", 1)
        .add_col_line("Notes", "#ffffff", 2)
        .add_col_line("Lumio V2", "#ffffff", 0)
        .add_col_line("Shipped", "#ffffff", 1)
        .add_col_line("Internship project", "#ffffff", 2)
        .segment_ratios(1, vec![1, 1, 2])
        .build()
        .display();

    println!("\n\n Extra tests, okay to tun off for a while");

    Boxy::builder()
        .box_type(BoxType::Double)
        .color("#00ffff")
        .align(BoxAlign::Center)
        .padding(BoxPad::uniform(1), BoxPad::vh(1, 2))
        .add_segment("Hello, boxy-cli!", "#ffffff", BoxAlign::Center)
        .add_line("A terminal box library for Rust.", "#aaaaaa")
        .add_segment("Another section", "#f19356", BoxAlign::Right)
        .build()
        .display();

    // Single-column header over a 3-column table — shows ┼/┴ junction chars
    let mut b = Boxy::new(BoxType::Single, "#00ffff");
    b.add_text_sgmt("Project Status", "#ffffff", BoxAlign::Center);
    b.add_col_text_sgmt(BoxAlign::Left, 3);
    b.add_col_text_line("Name", "#aaaaaa", &0);
    b.add_col_text_line("Status", "#aaaaaa", &1);
    b.add_col_text_line("Notes", "#aaaaaa", &2);
    b.add_col_text_line("Lumio V2", "#ffffff", &0);
    b.add_col_text_line("Shipped", "#32CD32", &1);
    b.add_col_text_line("Internship project", "#ffffff", &2);
    b.add_col_text_line("ShellDeck", "#ffffff", &0);
    b.add_col_text_line("In progress", "#ffaa00", &1);
    b.add_col_text_line("Father board PCB", "#ffffff", &2);
    b.set_segment_ratios(1, vec![1, 1, 2]);
    b.display();

    let mut b = Boxy::new(BoxType::Bold, "#00ffff");
    b.add_text_sgmt(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor \
                 incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
                 exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
                "#ffffff",
                BoxAlign::Left,
            );
    b.add_text_sgmt(
        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium \
                 doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore \
                 veritatis et quasi architecto beatae vitae dicta sunt explicabo.",
        "#ffffff",
        BoxAlign::Left,
    );
    b.add_text_sgmt("Hello There", "#ffffff", BoxAlign::Center);
    b.display();

    let mut b = Boxy::new(BoxType::Double, "#bfff00");
    b.add_text_sgmt(
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor \
                     incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
                     exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
                    "#ffffff",
                    BoxAlign::Left,
                );
    b.add_text_sgmt(
        "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium \
                     doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore \
                     veritatis et quasi architecto beatae vitae dicta sunt explicabo.",
        "#ffffff",
        BoxAlign::Left,
    );
    b.add_text_sgmt("Hello There", "#ffffff", BoxAlign::Center);
    b.display();
    let ok = b.render(termsize::get().unwrap().cols as usize);
    println!("{}", ok.join("\n"));
}
