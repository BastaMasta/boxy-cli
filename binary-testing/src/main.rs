use binary_testing::bling::*;

// Index 0 for full light
// Index 1 for double horizonta and light vertical
// Index 2 for light horizonta and double vertical
// Index 3 for full double


fn main() {
    let mut box1 = Boxy::new(BoxType::Classic,"#00ffff");
    // Post line insert check
    println!("{:?}", box1);
    box1.add_text_sgmt("Hello There Mateojablij trhwesoiuethj 0piswe hjgtgoise jgtowie3thj q3o-oitujpwiej toiq 0iweeh gt owjtpiewrwh WOKWRHJ JRQWE4IHYNE5R bfg oiwhf apeih aepih aepih aepihetm wf[ohgwlMRF [POWQWRF]] [OJTQEA [OJ]]OJBDGISUDBG SIUGRG OGUFOSIJGOSN SOGUIHSGIORNGR ORIRHGOSJRNGOIJRG OPIFGHRPGNPERIJG ORIRGRPIGNERPGOSJH ", "#ffff");
    box1.add_text_sgmt("Hello There Mateojablij trhwesoiuethj 0piswe hjgtgoise jgtowie3thj q3o-oitujpwiej toiq 0iweeh gt owjtpiewrwh WOKWRHJ JRQWE4IHYNE5R bfg oiwhf apeih aepih aepih aepihetm wf[ohgwlMRF [POWQWRF]] [OJTQEA [OJ]]OJBDGISUDBG SIUGRG OGUFOSIJGOSN SOGUIHSGIORNGR ORIRHGOSJRNGOIJRG OPIFGHRPGNPERIJG ORIRGRPIGNERPGOSJH ", "#ffff");
    // Post line insert check
    println!("{:?}", box1);
    println!("\nTEXTBOX:");
    box1.display();
}


// const PEICES : [[&str; 11]; 4] = [["┌", "┐", "└", "┘", "│", "─", "├", "┼", "┤", "┬", "┴"], 
//                                   ["╒", "╕", "╘", "╛", "│", "═", "╞", "╪", "╡", "╤", "╧"],
//                                   ["╓", "╖", "╙", "╜", "║", "─", "╟", "╫", "╢", "╥", "╨"],
//                                   ["╔", "╗", "╚", "╝", "║", "═", "╠", "╬", "╣", "╦", "╩"]];


// fn boxy(data: &str, color : &str) {
//     let term = termsize::get().unwrap();
//     let terminal_size = (term.cols as usize) - 20;
//     let col_truevals = HexColor::parse(color).unwrap();
//     let mut processed_data = String::from (data.trim());
//     processed_data.push(' ');
//     let whitespace_indices_temp = processed_data.match_indices(" ").collect::<Vec<_>>();
//     let mut ws_indices = Vec::new();
//     for (i,_) in whitespace_indices_temp {
//         ws_indices.push(i);
//     }
//     println!("{:?}", nearest_whitespace(&mut ws_indices, &terminal_size, 0));
//     println!("{:?}", ws_indices);
//     print!("{}", PEICES[0][0].truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
//     for _ in 0..=terminal_size {
//         print!("{}", PEICES[0][5].truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
//     }
//     println!("{}", PEICES[0][1].truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
//     print!("{}", PEICES[0][4].truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
//     let padded_string = format!("   {:<width$}", 
//                                 &processed_data[0..nearest_whitespace(&mut ws_indices, &(&terminal_size-5), 0)],
//                                 width=terminal_size-2);
//     print!("{}", padded_string);
//     print!("{}", PEICES[0][4].truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
//     println!("");
//     print!("{}", PEICES[0][2].truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
//     for _ in 0..=terminal_size {
//         print!("{}", PEICES[0][5].truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
//     }
//     println!("{}", PEICES[0][3].truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
    

// }

// fn nearest_whitespace(map: &mut Vec<usize>, term_size: &usize, start_index: usize) -> usize {
//     let mut prev = 0;
//     let mut curr = 0;
//     for i in &mut map[start_index..] {
//         curr = *i;
//         if curr > *term_size+1 {
//             return prev;
//         } else {
//             prev = curr;
//         }
//     }
//     return curr;
// }