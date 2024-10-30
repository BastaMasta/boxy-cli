use colored::Colorize;

fn main() {
    boxy(" Hello There BOIII");
}


fn boxy(data: &str) {
    let terminal_size = 20;
    let whitespace_indices_temp = data.trim().match_indices(" ").collect::<Vec<_>>();
    let mut ws_indices = Vec::new();
    for (i,_) in whitespace_indices_temp {
        ws_indices.push(i);
    }
    println!("{:?}", nearest_whitespace(&mut ws_indices, &terminal_size, 0));
    println!("{:?}", ws_indices);
    let light = ["┌", "┐", "└", "┘", "│", "─", "├", "┼", "┤"];
    let mut padded_string = String::from(" ");
    print!("{}", light[0].blue());
    for _ in 0..=terminal_size {
        print!("{}", light[5].blue());
    }
    println!("{}", light[1].blue());
    for __ in 0..5 {
        print!("{}", light[4].blue());
        for _ in 0..=terminal_size {
            print!(" "); 
        };
        print!("{}", light[4].blue());
        println!("");
    }
    print!("{}", light[4].blue());
    padded_string = format!("{:<20}", &data.trim()[0..nearest_whitespace(&mut ws_indices, &terminal_size, 0)]);
    print!("{}", padded_string);
    print!("{}", light[4].blue());
    println!("");

}

fn nearest_whitespace(map: &mut Vec<usize>, term_size: &usize, start_index: usize) -> usize {
    let mut prev = 0;
    let mut curr = 0;
    for i in &mut map[start_index..] {
        curr = *i;
        if curr > (*term_size) {
            return prev;
        } else {
            prev = curr;
        }
    }
    return curr;
}