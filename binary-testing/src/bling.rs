use colored::Colorize;
use hex_color::HexColor;

#[derive(Debug)]
pub struct BoxTemplates {
    top_left : char,
    top_right : char,
    bottom_left : char,
    bottom_right : char,
    vertical : char,
    horizontal : char,
    // LeftT : char,
    // RightT : char,
    // UpperT : char,
    // LowerT : char,
    // Cross : char,
}

pub const SINGLE_TEMPLATE : BoxTemplates = BoxTemplates {
    top_left : '┌',
    top_right : '┐',
    bottom_left : '└',
    bottom_right : '┘',
    vertical : '│',
    horizontal : '─',
    // LeftT : '├',
    // RightT : '┤',
    // UpperT : '┬',
    // LowerT : '┴',
    // Cross : '┼',
};

const DOUB_H_TEMPLATE : BoxTemplates = BoxTemplates {
    top_left : '╒',
    top_right : '╕',
    bottom_left : '╘',
    bottom_right : '╛',
    vertical : '│',
    horizontal : '═',
    // LeftT : '╞',
    // RightT : '╡',
    // UpperT : '╤',
    // LowerT : '╧',
    // Cross : '╪',
};

const DOUB_V_TEMPLATE : BoxTemplates = BoxTemplates {
    top_right : '╖',
	top_left : '╓',
	bottom_right : '╜',
	bottom_left : '╙',
	horizontal : '─',
	vertical : '║',
};

const DOUBLE_TEMPLATE : BoxTemplates = BoxTemplates {
    top_right : '╗',
    top_left : '╔',
    bottom_right : '╝',
    bottom_left : '╚',
    horizontal : '═',
    vertical : '║',
};

const ROUNDED_TEMPLATE : BoxTemplates = BoxTemplates {
    top_right : '╮',
	top_left : '╭',
	bottom_right : '╯',
	bottom_left : '╰',
	horizontal : '─',
	vertical : '│',
};

const BOLD_TEMPLATE : BoxTemplates = BoxTemplates {
    top_right : '┓',
	top_left : '┏',
	bottom_right : '┛',
	bottom_left : '┗',
	horizontal : '━',
	vertical : '┃',
};

const CLASSIC_TEMPLATE : BoxTemplates = BoxTemplates {
    top_right : '+',
    top_left : '+',
    bottom_right : '+',
    bottom_left : '+',
    horizontal : '-',
    vertical : '|',
};

#[derive(Debug)]
pub struct Boxy {
    data : Vec<String>,
    box_col : String,
    colors : Vec<String>,
    divy : Vec<usize>,

}

impl Boxy {
    pub fn new(box_color : &str) -> Self {
        Boxy{
            data : Vec::<String>::new(),
            box_col : (&box_color).to_string(),
            colors : Vec::<String>::new(),
            divy : Vec::<usize>::new(),
        }
    }

    pub fn add_line(&mut self, data_string : &str, color : &str) {
        self.data.push(String::from(data_string));
        self.colors.push(String::from(color));
    }

    pub fn display(&mut self) {
        let term = termsize::get().unwrap();
        let terminal_size = (term.cols as usize) - 20;
        let col_truevals = HexColor::parse(&self.box_col).unwrap();
        print!("{}", BOLD_TEMPLATE.top_left.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
        for _ in 0..=terminal_size {
            print!("{}", BOLD_TEMPLATE.horizontal.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
        }
        println!("{}", BOLD_TEMPLATE.top_right.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
        
                
//          
    }
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

fn recur_whitespace_printing(map: &mut Vec<usize>, term_size: &usize, start_index: usize) {
    print!("{}", BOLD_TEMPLATE.vertical.to_string().blue());
} 

pub fn add(left: u64, right: u64) -> u64 {
    println!("{:?}", SINGLE_TEMPLATE);
    left + right
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
