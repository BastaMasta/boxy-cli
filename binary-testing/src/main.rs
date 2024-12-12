use binary_testing::bling::*;
use binary_testing::boxy;

fn main() {
    let mut box1 = Boxy::new(BoxType::Bold,"#00ffff");
    box1.add_text_sgmt("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.", "#663399");
    box1.add_text_sgmt("Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur.", "#bfff00");
    box1.add_text_sgmt("Hello Theree", "#00ffff");
    box1.set_align(BoxAlign::Center);
    println!("\nTEXTBOX:");
    box1.display();
    let mut box2 = boxy!(boxtype: BoxType::Double, color:"#00ffff", align:BoxAlign::Center);
    // box2.add_text_sgmt("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.", "#ffff");
    box2.add_text_sgmt("Hello Theree BIGG Bo sdsdg gdsi", "#ffff");
    box2.set_padding_ext(54);
    box2.display();

}
