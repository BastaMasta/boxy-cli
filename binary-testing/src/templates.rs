#[derive(Debug)]
pub struct BoxTemplates {
    pub top_left : char,
    pub top_right : char,
    pub bottom_left : char,
    pub bottom_right : char,
    pub vertical : char,
    pub horizontal : char,
    pub left_t : char,
    pub right_t : char,
    pub upper_t : char,
    pub lower_t : char,
    pub cross : char,
}

pub const SINGLE_TEMPLATE : BoxTemplates = BoxTemplates {
    top_left : '┌',
    top_right : '┐',
    bottom_left : '└',
    bottom_right : '┘',
    vertical : '│',
    horizontal : '─',
    left_t : '├',
    right_t : '┤',
    upper_t : '┬',
    lower_t : '┴',
    cross : '┼',
};

pub const DOUB_H_TEMPLATE : BoxTemplates = BoxTemplates {
    top_left : '╒',
    top_right : '╕',
    bottom_left : '╘',
    bottom_right : '╛',
    vertical : '│',
    horizontal : '═',
    left_t : '╞',
    right_t : '╡',
    upper_t : '╤',
    lower_t : '╧',
    cross : '╪',
};

pub const DOUB_V_TEMPLATE : BoxTemplates = BoxTemplates {
    top_right : '╖',
    top_left : '╓',
    bottom_right : '╜',
    bottom_left : '╙',
    horizontal : '─',
    vertical : '║',
    left_t : '╟',
    right_t : '╢',
    upper_t : '╥',
    lower_t : '╨',
    cross : '╫',
};

pub const DOUBLE_TEMPLATE : BoxTemplates = BoxTemplates {
    top_right : '╗',
    top_left : '╔',
    bottom_right : '╝',
    bottom_left : '╚',
    horizontal : '═',
    vertical : '║',
    left_t : '╠',
    right_t : '╣',
    upper_t : '╦',
    lower_t : '╩',
    cross : '╬',
};

pub const ROUNDED_TEMPLATE : BoxTemplates = BoxTemplates {
    top_right : '╮',
    top_left : '╭',
    bottom_right : '╯',
    bottom_left : '╰',
    horizontal : '─',
    vertical : '│',
    left_t : '├',
    right_t : '┤',
    upper_t : '┬',
    lower_t : '┴',
    cross : '┼',
};

pub const BOLD_TEMPLATE : BoxTemplates = BoxTemplates {
    top_right : '┓',
    top_left : '┏',
    bottom_right : '┛',
    bottom_left : '┗',
    horizontal : '━',
    vertical : '┃',
    left_t : '┣',
    right_t : '┫',
    upper_t : '┳',
    lower_t : '┻',
    cross : '╋',
};

pub const CLASSIC_TEMPLATE : BoxTemplates = BoxTemplates {
    top_right : '+',
    top_left : '+',
    bottom_right : '+',
    bottom_left : '+',
    horizontal : '-',
    vertical : '┇',
    left_t : '+',
    right_t : '+',
    upper_t : '+',
    lower_t : '+',
    cross : '+',
};

pub const BOLD_CORNERS_TEMPLATE : BoxTemplates = BoxTemplates {
    top_right : '┓',
    top_left : '┏',
    bottom_right : '┛',
    bottom_left : '┗',
    horizontal : '─',
    vertical : '│',
    left_t : '├',
    right_t : '┤',
    upper_t : '┬',
    lower_t : '┴',
    cross : '┼',
};