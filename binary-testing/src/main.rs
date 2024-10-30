use colored::Colorize;

// Index 0 for full light
// Index 1 for double horizonta and light vertical
// Index 2 for light horizonta and double vertical
// Index 3 for full double

const PEICES : [[&str; 11]; 4] = [["┌", "┐", "└", "┘", "│", "─", "├", "┼", "┤", "┬", "┴"], 
                                  ["╒", "╕", "╘", "╛", "│", "═", "╞", "╪", "╡", "╤", "╧"],
                                  ["╓", "╖", "╙", "╜", "║", "─", "╟", "╫", "╢", "╥", "╨"],
                                  ["╔", "╗", "╚", "╝", "║", "═", "╠", "╬", "╣", "╦", "╩"]];

fn main() {
    boxy(" Hello There BOIII");
}


fn boxy(data: &str) {
    let term = termsize::get().unwrap();
    let terminal_size = (term.cols as usize) - 20;
    let mut processed_data = String::from (data.trim());
    processed_data.push(' ');
    let whitespace_indices_temp = processed_data.match_indices(" ").collect::<Vec<_>>();
    let mut ws_indices = Vec::new();
    for (i,_) in whitespace_indices_temp {
        ws_indices.push(i);
    }
    println!("{:?}", nearest_whitespace(&mut ws_indices, &terminal_size, 0));
    println!("{:?}", ws_indices);
    print!("{}", PEICES[0][0].blue());
    for _ in 0..=terminal_size {
        print!("{}", PEICES[0][5].blue());
    }
    println!("{}", PEICES[0][1].blue());
    print!("{}", PEICES[0][4].blue());
    let padded_string = format!("   {:<width$}", 
                                &processed_data[0..nearest_whitespace(&mut ws_indices, &(&terminal_size-5), 0)],
                                width=terminal_size-2);
    print!("{}", padded_string);
    print!("{}", PEICES[0][4].blue());
    println!("");
    print!("{}", PEICES[0][2].blue());
    for _ in 0..=terminal_size {
        print!("{}", PEICES[0][5].blue());
    }
    println!("{}", PEICES[0][3].blue());
    

}

fn nearest_whitespace(map: &mut Vec<usize>, term_size: &usize, start_index: usize) -> usize {
    let mut prev = 0;
    let mut curr = 0;
    for i in &mut map[start_index..] {
        curr = *i;
        if curr > *term_size+1 {
            return prev;
        } else {
            prev = curr;
        }
    }
    return curr;
}