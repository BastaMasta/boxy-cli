//! The main datatypes and enums used by the `Boxy` struct.

use std::{borrow::Cow, fmt::Display};

/// Defines the border style for the text box.
///
/// Each variant represents a different visual style for the box borders.
///
/// # Examples
///
/// ```
/// use boxy_cli::prelude::*;
///
/// let mut box1 = Boxy::new(BoxType::Double, "#00ffff");
/// let mut box2 = Boxy::new(BoxType::Rounded, "#00ffff");
/// let mut box3 = Boxy::new(BoxType::Bold, "#00ffff");
/// ```
#[derive(Debug, Default)]
pub enum BoxType {
    /// Simple ASCII-style box using `+` for corners and `-` for borders
    Classic,
    /// Default style using single-line Unicode box drawing characters
    #[default]
    Single,
    /// Double horizontal lines with single vertical lines
    DoubleHorizontal,
    /// Double vertical lines with single horizontal lines
    DoubleVertical,
    /// Double lines for all borders
    Double,
    /// Thicker/bold lines for all borders
    Bold,
    /// Box with rounded corners
    Rounded,
    /// Box with bold corners and normal edges
    BoldCorners,
    /// Box with no borders (invisible)
    Empty,
}

// Added Display Fucntion to resolve type errors in the macro
impl Display for BoxType {
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
            BoxType::Empty => "empty".to_string(),
        };
        write!(f, "{}", str)
    }
}

/// Specifies the alignment of text within the text box or the box itself within the terminal.
///
/// This enum is used in two contexts:
/// 1. To control the alignment of text segments within the box
/// 2. To control the alignment of the entire box within the terminal width
///
/// # Examples
///
/// ```
/// use boxy_cli::prelude::*;
///
/// let mut my_box = Boxy::new(BoxType::Single, "#00ffff");
///
/// // Center the box in the terminal
/// my_box.set_align(BoxAlign::Center);
///
/// // Add a left-aligned text segment
/// my_box.add_text_sgmt("Left aligned text", "#ffffff", BoxAlign::Left);
///
/// // Add a right-aligned text segment
/// my_box.add_text_sgmt("Right aligned text", "#ffffff", BoxAlign::Right);
/// ```
#[derive(Debug, Default)]
pub enum BoxAlign {
    /// Align to the left side
    Left,
    /// Center alignment (default)
    #[default]
    Center,
    /// Align to the right side
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

/// Represents padding values for the text box in all four directions.
///
/// `BoxPad` is used to specify padding between:
/// - The box border and the contained text (internal padding)
/// - The terminal edges and the box itself (external padding)
///
/// # Examples
///
/// ```
/// use boxy_cli::prelude::*;
///
/// // Create uniform padding of 2 spaces on all sides
/// let uniform_padding = BoxPad::uniform(2);
///
/// // Create custom padding for each side
/// let custom_padding = BoxPad::from_tldr(1, 3, 1, 3); // top, left, down, right
///
/// // Create horizontal/vertical padding
/// let h_v_padding = BoxPad::vh(1, 3); // 1 vertical, 3 horizontal
/// ```
#[derive(Debug)]
pub struct BoxPad {
    /// Padding at the top
    pub top: usize,
    /// Padding at the bottom
    pub down: usize,
    /// Padding on the left side
    pub left: usize,
    /// Padding on the right side
    pub right: usize,
}

impl Default for BoxPad {
    fn default() -> Self {
        Self::new()
    }
}

impl BoxPad {
    /// Creates a new `BoxPad` instance with zero padding on all sides.
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let padding = BoxPad::new();
    /// // Equivalent to BoxPad { top: 0, down: 0, left: 0, right: 0 }
    /// ```
    pub fn new() -> Self {
        BoxPad {
            top: 0,
            down: 0,
            left: 0,
            right: 0,
        }
    }
    /// Creates a new `BoxPad` with specific values for each side.
    ///
    /// The parameter order follows the mnemonic "tldr" (top, left, down, right).
    ///
    /// # Arguments
    ///
    /// * `top` - Padding at the top
    /// * `left` - Padding on the left side
    /// * `down` - Padding at the bottom
    /// * `right` - Padding on the right side
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// // Create padding with different values on each side
    /// let padding = BoxPad::from_tldr(2, 4, 2, 4);
    /// // top: 2, left: 4, down: 2, right: 4
    /// ```
    pub fn from_tldr(top: usize, left: usize, down: usize, right: usize) -> Self {
        BoxPad {
            top,
            down,
            left,
            right,
        }
    }

    /// Creates a new `BoxPad` with the same padding value on all sides.
    ///
    /// # Arguments
    ///
    /// * `pad` - The padding value to use for all sides
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// // Create uniform padding of 3 spaces on all sides
    /// let padding = BoxPad::uniform(3);
    /// // Equivalent to BoxPad { top: 3, down: 3, left: 3, right: 3 }
    /// ```
    pub fn uniform(pad: usize) -> Self {
        BoxPad {
            top: pad,
            down: pad,
            left: pad,
            right: pad,
        }
    }
    /// Creates a new `BoxPad` with separate values for vertical and horizontal padding.
    ///
    /// This is a convenience method that applies the same padding to top/bottom
    /// and left/right sides.
    ///
    /// # Arguments
    ///
    /// * `vertical` - Padding for top and bottom
    /// * `horizontal` - Padding for left and right
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// // Create padding with 1 space vertically and 3 spaces horizontally
    /// let padding = BoxPad::vh(1, 3);
    /// // Equivalent to BoxPad { top: 1, down: 1, left: 3, right: 3 }
    /// ```
    pub fn vh(vertical: usize, horizontal: usize) -> Self {
        BoxPad {
            top: vertical,
            down: vertical,
            left: horizontal,
            right: horizontal,
        }
    }
    /// returns the total padidng on either side. used for text wrapping and display time calculations
    pub fn lr(&self) -> usize {
        self.right + self.left
    }
}

#[derive(Debug)]
pub enum SegType <'a>{
    Single(Vec<Cow<'a, str>>),
    Columnar(Vec<Vec<Cow<'a, str>>>)
}
