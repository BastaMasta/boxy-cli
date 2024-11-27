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
    int_padding: usize,
    ext_padding: usize,

}

impl Boxy {
    pub fn new(box_color : &str) -> Self {
        Boxy{
            data : Vec::<String>::new(),
            box_col : (&box_color).to_string(),
            colors : Vec::<String>::new(),
            divy : Vec::<usize>::new(),
            int_padding: 5 as usize,
            ext_padding: 5 as usize,

        }
    }

    pub fn add_line(&mut self, data_string : &str, color : &str) {
        self.data.push(String::from(data_string));
        self.colors.push(String::from(color));
    }

    pub fn display(&mut self) {
        // Initialising Display Variables
        let term = termsize::get().unwrap();
        let terminal_size = (term.cols as usize) - 20 - 2*self.ext_padding;
        let col_truevals = HexColor::parse(&self.box_col).unwrap();

        // Processing data ad setting up whitespaces map
        let mut processed_data = String::from (self.data[0].trim());
        processed_data.push(' ');
        let whitespace_indices_temp = processed_data.match_indices(" ").collect::<Vec<_>>();
        let mut ws_indices = Vec::new();
        for (i,_) in whitespace_indices_temp {
            ws_indices.push(i);
        }

        // Actually printing shiet
        print!("{:>width$}", BOLD_TEMPLATE.top_left.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b), width=self.ext_padding+1);
        for _ in 0..terminal_size {
            print!("{}", BOLD_TEMPLATE.horizontal.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
        }
        println!("{}", BOLD_TEMPLATE.top_right.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
        
        recur_whitespace_printing(&processed_data, &mut ws_indices, &(terminal_size-self.int_padding), 0 as usize, &col_truevals, &self.ext_padding, &self.int_padding);
        
        print!("{:>width$}", BOLD_TEMPLATE.bottom_left.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b), width=self.ext_padding+1);
        for _ in 0..terminal_size {
            print!("{}", BOLD_TEMPLATE.horizontal.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
        }
        println!("{}", BOLD_TEMPLATE.bottom_right.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
        
    }
}

fn nearest_whitespace(map: &mut Vec<usize>, term_size: &usize, start_index: usize) -> usize {
    let mut prev = 0;
    let mut curr = 0;
    let mut pos = 0;
    if start_index == map[map.len()-1] {
        return start_index;
    }
    for j in &mut *map {
        if *j < start_index {
            pos += 1;
        }
    }
    for i in &mut map[pos..] {
        curr = *i;
        if curr > *term_size+1 {
            return prev;
        } else {
            prev = curr;
        }
    }
    return prev;
}

fn recur_whitespace_printing(data:&str ,map: &mut Vec<usize>, term_size: &usize, start_index: usize, boxcol: &HexColor, ext_padding: &usize, int_padding: &usize) {
    print!("{:>width$}", BOLD_TEMPLATE.vertical.to_string().truecolor(boxcol.r, boxcol.g, boxcol.b), width=*ext_padding+1);
    let next_ws = nearest_whitespace(map, term_size, start_index);
    print!("{:<pad$}", " ", pad=*int_padding);
    print!("{:<width$}", &data[start_index..next_ws], width=term_size,);
    print!("{}", BOLD_TEMPLATE.vertical.to_string().truecolor(boxcol.r, boxcol.g, boxcol.b));
    println!(" ");
    if next_ws < (map[map.len()-1]) {
        recur_whitespace_printing(data, map, term_size, next_ws, boxcol, ext_padding, int_padding);
    }
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
