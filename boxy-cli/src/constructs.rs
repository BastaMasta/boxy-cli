use std::fmt::Display;

/// Type of border for the TextBox
///
/// Total of 8 types: Classic, Single, DoubleHorizontal, DoubleVertical, Double, Bold, Rounded and BoldCorners
// TextBox Type Enums
#[derive(Debug)]
pub enum BoxType{
    Classic,
    Single,
    DoubleHorizontal,
    DoubleVertical,
    Double,
    Bold,
    Rounded,
    BoldCorners
}

// Added Display Fucntion to resolve type errors in the macro
impl Display for BoxType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            BoxType::Classic => "classic".to_string(),
            BoxType::Single => "single".to_string(),
            BoxType::DoubleHorizontal => "double_horizontal".to_string(),
            BoxType::DoubleVertical => "double_vertical".to_string(),
            BoxType::Double => "double".to_string(),
            BoxType::Bold => "bold".to_string(),
            BoxType::Rounded => "rounded".to_string(),
            BoxType::BoldCorners => "bold_corners".to_string(),
        };
        write!(f, "{}", str)
    }
}

// Alignment Enums
/// Type of alignment for the text inside the TextBox
///
/// Total of 3 types: Left, Center and Right
#[derive(Debug)]
pub enum BoxAlign {
    Left,
    Center,
    Right,
}

// Added Display Fucntion to resolve type errors in the macro
impl Display for BoxAlign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BoxAlign::Left => "left".to_string(),
            BoxAlign::Center => "center".to_string(),
            BoxAlign::Right => "right".to_string(),
        };
        write!(f, "{}", str)
    }
}


/// Padding Struct for the TextBox
#[derive(Debug)]
pub struct BoxPad {
    pub top: usize,
    pub down: usize,
    pub left: usize,
    pub right: usize,
}

impl Default for BoxPad {
    fn default() -> Self {
        Self::new()
    }
}

impl BoxPad {
    pub fn new() -> Self {
        BoxPad{
            top: 0,
            down: 0,
            left: 0,
            right: 0
        }
    }
    /// Creates a new BoxPad Struct with the given padding values in order: top, left, down, right (tldr)
    pub fn from_tldr(top: usize, left: usize, down: usize, right: usize) -> Self {
        BoxPad{
            top,
            down,
            left,
            right
        }
    }

    /// Creates a new BoxPad Struct with uniform padding values on all sides
    pub fn uniform(pad: usize) -> Self{
        BoxPad{
            top: pad,
            down: pad,
            left: pad,
            right: pad
        }
    }
    /// Creates a new BoxPad Struct with the given padding values in order: vertical, horizontal
    pub fn vh(vertical: usize, horizontal: usize) -> Self{
        BoxPad{
            top: vertical,
            down: vertical,
            left: horizontal,
            right: horizontal
        }
    }
    /// returns the total padidng on either side. used for text wrapping and display time calculations
    pub fn lr(&self) -> usize{
        self.right + self.left
    }
}