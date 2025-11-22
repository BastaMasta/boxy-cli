//! The structs for the box border characters
#[allow(dead_code)]
pub(crate) struct BoxTemplates {
    pub(crate) top_left: char,
    pub(crate) top_right: char,
    pub(crate) bottom_left: char,
    pub(crate) bottom_right: char,
    pub(crate) vertical: char,
    pub(crate) horizontal: char,
    pub(crate) left_t: char,
    pub(crate) right_t: char,
    pub(crate) upper_t: char,
    pub(crate) lower_t: char,
    pub(crate) cross: char,
}

pub(crate) const SINGLE_TEMPLATE: BoxTemplates = BoxTemplates {
    top_left: '┌',
    top_right: '┐',
    bottom_left: '└',
    bottom_right: '┘',
    vertical: '│',
    horizontal: '─',
    left_t: '├',
    right_t: '┤',
    upper_t: '┬',
    lower_t: '┴',
    cross: '┼',
};

pub(crate) const DOUB_H_TEMPLATE: BoxTemplates = BoxTemplates {
    top_left: '╒',
    top_right: '╕',
    bottom_left: '╘',
    bottom_right: '╛',
    vertical: '│',
    horizontal: '═',
    left_t: '╞',
    right_t: '╡',
    upper_t: '╤',
    lower_t: '╧',
    cross: '╪',
};

pub(crate) const DOUB_V_TEMPLATE: BoxTemplates = BoxTemplates {
    top_right: '╖',
    top_left: '╓',
    bottom_right: '╜',
    bottom_left: '╙',
    horizontal: '─',
    vertical: '║',
    left_t: '╟',
    right_t: '╢',
    upper_t: '╥',
    lower_t: '╨',
    cross: '╫',
};

pub(crate) const DOUBLE_TEMPLATE: BoxTemplates = BoxTemplates {
    top_right: '╗',
    top_left: '╔',
    bottom_right: '╝',
    bottom_left: '╚',
    horizontal: '═',
    vertical: '║',
    left_t: '╠',
    right_t: '╣',
    upper_t: '╦',
    lower_t: '╩',
    cross: '╬',
};

pub(crate) const ROUNDED_TEMPLATE: BoxTemplates = BoxTemplates {
    top_right: '╮',
    top_left: '╭',
    bottom_right: '╯',
    bottom_left: '╰',
    horizontal: '─',
    vertical: '│',
    left_t: '├',
    right_t: '┤',
    upper_t: '┬',
    lower_t: '┴',
    cross: '┼',
};

pub(crate) const BOLD_TEMPLATE: BoxTemplates = BoxTemplates {
    top_right: '┓',
    top_left: '┏',
    bottom_right: '┛',
    bottom_left: '┗',
    horizontal: '━',
    vertical: '┃',
    left_t: '┣',
    right_t: '┫',
    upper_t: '┳',
    lower_t: '┻',
    cross: '╋',
};

pub(crate) const CLASSIC_TEMPLATE: BoxTemplates = BoxTemplates {
    top_right: '+',
    top_left: '+',
    bottom_right: '+',
    bottom_left: '+',
    horizontal: '-',
    vertical: '┇',
    left_t: '+',
    right_t: '+',
    upper_t: '+',
    lower_t: '+',
    cross: '+',
};

pub(crate) const BOLD_CORNERS_TEMPLATE: BoxTemplates = BoxTemplates {
    top_right: '┓',
    top_left: '┏',
    bottom_right: '┛',
    bottom_left: '┗',
    horizontal: '─',
    vertical: '│',
    left_t: '├',
    right_t: '┤',
    upper_t: '┬',
    lower_t: '┴',
    cross: '┼',
};

pub(crate) const EMPTY_TEMPLATE: BoxTemplates = BoxTemplates {
    top_right: ' ',
    top_left: ' ',
    bottom_right: ' ',
    bottom_left: ' ',
    horizontal: ' ',
    vertical: ' ',
    left_t: ' ',
    right_t: ' ',
    upper_t: ' ',
    lower_t: ' ',
    cross: ' ',
};
