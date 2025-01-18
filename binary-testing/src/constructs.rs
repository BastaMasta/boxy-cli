use std::fmt::Display;

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