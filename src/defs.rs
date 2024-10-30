
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
