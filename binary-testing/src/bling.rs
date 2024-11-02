
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
    colors : Vec<String>,
    divy : Vec<usize>,

}

impl Boxy {
    pub fn new() -> Self {
        Boxy{
            data : Vec::<String>::new(),
            colors : Vec::<String>::new(),
            divy : Vec::<usize>::new(),
        }
    }

    pub fn add_line(&mut self, data_string : &str, color : &str) {
        self.data.push(String::from(data_string));
        self.colors.push(String::from(color));
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
