//! The main crate logic

use std::borrow::Cow;
use colored::{Color, Colorize};
use hex_color::HexColor;
use crate::templates::*;
use crate::constructs::*;


/// The main struct that represents a text box for CLI display.
///
/// `Boxy` contains all the configuration and content needed to render a styled text box
/// in the terminal, including borders, text content, colors, padding, and alignment options.
///
/// # Examples
///
/// ```
/// use boxy_cli::prelude::*;
///
/// // Create a simple text box
/// let mut my_box = Boxy::new(BoxType::Double, "#00ffff");
/// my_box.add_text_sgmt("Hello, World!", "#ffffff", BoxAlign::Center);
/// my_box.display();
/// ```
#[derive(Debug)]
pub struct Boxy<'a> {
    type_enum: BoxType,
    data: Vec<Vec<Cow<'a, str>>>,
    sect_count: usize,
    box_col: String,
    colors: Vec<Vec<Cow<'a, str>>>,
    int_padding: BoxPad,
    ext_padding: BoxPad,
    align: BoxAlign,
    seg_align: Vec<BoxAlign>,
    fixed_width: usize,
    fixed_height: usize,
    seg_cols_count: Vec<usize>,
    seg_cols_ratio: Vec<Vec<usize>>,
    tot_seg: usize,
    terminal_width_offset: i32,
}

// Default struct values for the textbox
impl Default for Boxy<'_> {
    fn default() -> Self {
        Self {
            type_enum: BoxType::Single,
            data : Vec::<Vec<Cow<str>>>::new(),
            sect_count: 0usize,
            box_col: "#ffffff".to_string(),
            colors : Vec::<Vec<Cow<str>>>::new(),
            int_padding: BoxPad::new(),
            ext_padding: BoxPad::new(),
            align : BoxAlign::Left,
            seg_align: Vec::<BoxAlign>::new(),
            fixed_width: 0usize,
            fixed_height: 0usize,
            seg_cols_ratio: Vec::<Vec<usize>>::new(),
            seg_cols_count: Vec::<usize>::new(),
            tot_seg: 0usize,
            terminal_width_offset: -20
        }
    }
}


impl<'a> Boxy<'a> {
    /// Creates a new instance of the `Boxy` struct with the specified border type and color.
    ///
    /// # Arguments
    ///
    /// * `box_type` - The border style to use from the `BoxType` enum
    /// * `box_color` - A hex color code (e.g. "#00ffff") for the border color
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let mut my_box = Boxy::new(BoxType::Double, "#00ffff");
    /// ```
    pub fn new(box_type: BoxType, box_color : &str) -> Self {
        Boxy {
            type_enum: box_type,
            box_col: box_color.to_string(),
            ..Self::default()
        }
    }
    /// Returns a new `BoxyBuilder` to create a textbox using the builder pattern.
    ///
    /// The builder pattern provides a more fluent interface for configuring and creating a `Boxy` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let my_box = Boxy::builder()
    ///     .box_type(BoxType::Double)
    ///     .color("#00ffff")
    ///     .add_segment("Hello, World!", "#ffffff", BoxAlign::Center)
    ///     .build();
    /// ```
    pub fn builder() -> BoxyBuilder <'a> {
        BoxyBuilder::new()
    }

    /// Adds a new text segment/section to the textbox, separated by a horizontal divider.
    ///
    /// Each segment represents a distinct section of the textbox that will be separated by
    /// horizontal dividers. This method is typically used to add the first or later major
    /// sections of content.
    ///
    /// # Arguments
    ///
    /// * `data_string` - The text content for this segment
    /// * `color` - A hex color code (e.g. "#ffffff") for the text color
    /// * `text_align` - The alignment (left, center, right) for this text segment
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let mut my_box = Boxy::new(BoxType::Single, "#00ffff");
    /// my_box.add_text_sgmt("Header section", "#ffffff", BoxAlign::Center);
    /// my_box.add_text_sgmt("Content section", "#ffffff", BoxAlign::Left);
    /// ```
    pub fn add_text_sgmt(&mut self, data_string : &str, color : &str, text_align: BoxAlign) {
        self.data.push(vec![Cow::from(data_string.to_owned())]);
        self.colors.push(vec![Cow::from(String::from(color))]);
        self.seg_align.push(text_align);
        self.sect_count+=1;
    }

    /// Adds a new text line to the segment with a specific index.
    ///
    /// This method allows adding additional lines of text to an existing segment by specifying
    /// the segment's index. The new line will appear below the existing content in that segment.
    ///
    /// # Arguments
    ///
    /// * `data_string` - The text content to add
    /// * `color` - A hex color code (e.g. "#ffffff") for the text color
    /// * `seg_index` - The index of the segment to add this line to (0-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let mut my_box = Boxy::new(BoxType::Single, "#00ffff");
    /// my_box.add_text_sgmt("First segment", "#ffffff", BoxAlign::Left);
    /// my_box.add_text_sgmt("Second segment", "#ffffff", BoxAlign::Left);
    ///
    /// // Add a line to the first segment (index 0)
    /// my_box.add_text_line_indx("Additional line in first segment", "#32CD32", 0);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `seg_index` is out of bounds.
    pub fn add_text_line_indx(&mut self, data_string : &str, color: &str, seg_index : usize) {
        self.data[seg_index].push(Cow::from(data_string.to_owned()));
        self.colors[seg_index].push(Cow::from(String::from(color)));
    }
    
    /// Adds a new text line to the most recently added segment.
    ///
    /// This is a convenience method that adds a line of text to the last segment that was
    /// created, eliminating the need to specify the segment index.
    ///
    /// # Arguments
    ///
    /// * `data_string` - The text content to add
    /// * `color` - A hex color code (e.g. "#ffffff") for the text color
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let mut my_box = Boxy::new(BoxType::Single, "#00ffff");
    /// my_box.add_text_sgmt("Header", "#ffffff", BoxAlign::Center);
    /// my_box.add_text_line("Additional details below the header", "#32CD32");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if no segments have been added yet.
    pub fn add_text_line(&mut self, data_string : &str, color : &str) {
        self.data[self.sect_count-1].push(Cow::from(data_string.to_owned()));
        self.colors[self.sect_count-1].push(Cow::from(String::from(color)));
    }

    /// Sets the overall alignment of the textbox within the terminal.
    ///
    /// This controls the horizontal positioning of the entire textbox relative to the terminal width.
    /// It does not affect the alignment of text within the box segments.
    ///
    /// # Arguments
    ///
    /// * `align` - The alignment to use: `BoxAlign::Left`, `BoxAlign::Center`, or `BoxAlign::Right`
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let mut my_box = Boxy::new(BoxType::Single, "#00ffff");
    /// my_box.set_align(BoxAlign::Center); // Center the box in the terminal
    /// ```
    pub fn set_align(&mut self, align: BoxAlign) {
        self.align = align;
    }

    /// Sets the internal padding between the textbox border and its text content.
    ///
    /// Internal padding creates space between the border of the box and the text inside it.
    ///
    /// # Arguments
    ///
    /// * `int_padding` - A `BoxPad` instance specifying the padding values
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let mut my_box = Boxy::new(BoxType::Single, "#00ffff");
    ///
    /// // Set uniform padding of 2 spaces on all sides
    /// my_box.set_int_padding(BoxPad::uniform(2));
    ///
    /// // Or set different padding for each side (top, left, down, right)
    /// my_box.set_int_padding(BoxPad::from_tldr(1, 3, 1, 3));
    /// ```
    pub fn set_int_padding(&mut self, int_padding : BoxPad) {
        self.int_padding = int_padding;
    }
    /// Sets the external padding between the terminal edges and the textbox.
    ///
    /// External padding creates space between the edges of the terminal and the border of the box.
    /// This affects the positioning of the box within the terminal.
    ///
    /// # Arguments
    ///
    /// * `ext_padding` - A `BoxPad` instance specifying the padding values
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let mut my_box = Boxy::new(BoxType::Single, "#00ffff");
    /// 
    /// // Add 5 spaces of padding on all sides
    /// my_box.set_ext_padding(BoxPad::uniform(5));
    ///
    /// // Or set different padding for each side (top, left, down, right)
    /// my_box.set_ext_padding(BoxPad::from_tldr(0, 10, 0, 0));
    /// ```
    pub fn set_ext_padding(&mut self, ext_padding : BoxPad) {
        self.ext_padding = ext_padding;
    }
    /// Sets both internal and external padding for the textbox in a single call.
    ///
    /// This is a convenience method that combines `set_int_padding` and `set_ext_padding`.
    ///
    /// # Arguments
    ///
    /// * `ext_padding` - A `BoxPad` instance for the external padding (between terminal edges and box)
    /// * `int_padding` - A `BoxPad` instance for the internal padding (between box border and text)
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let mut my_box = Boxy::new(BoxType::Single, "#00ffff");
    ///
    /// // Set both internal and external padding
    /// my_box.set_padding(
    ///     BoxPad::from_tldr(1, 5, 1, 5), // external padding
    ///     BoxPad::uniform(2)            // internal padding
    /// );
    /// ```
    pub fn set_padding(&mut self, ext_padding : BoxPad, int_padding : BoxPad) {
        self.int_padding = int_padding;
        self.ext_padding = ext_padding;
    }

    /// Sets a fixed width for the textbox instead of dynamically sizing it to the terminal width.
    ///
    /// By default, the textbox automatically adjusts its width based on the terminal size.
    /// This method allows you to specify a fixed width instead.
    ///
    /// # Arguments
    ///
    /// * `width` - The desired width in characters (including borders)
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let mut my_box = Boxy::new(BoxType::Single, "#00ffff");
    /// my_box.set_width(50); // Fix the box width to 50 characters
    /// ```
    ///
    /// # Note
    ///
    /// Setting width to 0 returns to dynamic sizing based on terminal width.
    pub fn set_width(&mut self, width : usize) {
        self.fixed_width = width;
    }

    /// Sets a fixed height for the textbox by adding whitespace above and below the text.
    ///
    /// # Arguments
    ///
    /// * `height` - The desired height in characters (including borders)
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let mut my_box = Boxy::new(BoxType::Single, "#00ffff");
    /// my_box.set_height(20); // Set box height to 20 lines
    /// ```
    ///
    /// # Note
    ///
    /// This feature is experimental and may not work as expected in the current version.
    /// Setting height to 0 returns to dynamic sizing based on content.
    pub fn set_height(&mut self, height : usize) {
        self.fixed_height = height;
    }

    /// Sets the border type (box style).
    pub fn set_type(&mut self, box_type: BoxType) {
        self.type_enum = box_type;
    }

    /// Sets the border color using a hex string (e.g., "#00ffff").
    pub fn set_color(&mut self, color: &str) {
        self.box_col = color.to_string();
    }

    /// Sets the total segment count used by the macro; does not allocate content.
    /// This mirrors the previous macro behavior which only updated the count field.
    pub fn set_total_segments(&mut self, total: usize) {
        self.tot_seg = total;
    }

    /// Sets the size-ratio between segments when using vertical divisions
    ///
    /// This feature is still experimental and not yet implemented fully, and hence may not work in the current version of the crate.
    pub fn set_segment_ratios(&mut self, seg_index: usize, ratios: Vec<usize>) {
        if seg_index >= self.seg_cols_ratio.len() {
            self.seg_cols_ratio.resize(seg_index + 1, Vec::new());
        }
        self.seg_cols_ratio[seg_index] = ratios;
    }

    /// Renders and displays the textbox in the terminal.
    ///
    /// This method performs all the necessary calculations to render the textbox with the
    /// configured settings, including border style, colors, padding, and text content.
    /// It then prints the textbox to the standard output.
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let mut my_box = Boxy::new(BoxType::Double, "#00ffff");
    /// my_box.add_text_sgmt("Hello, World!", "#ffffff", BoxAlign::Center);
    /// my_box.display(); // Renders the box to the terminal
    /// ```
    ///
    /// # Note
    ///
    /// The appearance may vary depending on terminal support for colors and Unicode characters.
    pub fn display(&mut self) {

        // Initialising Display Variables

        let term_size = termsize::get().unwrap_or_else( ||
            {
                eprintln!("Failed to get terminal size, assuming default width of 80");
                termsize::Size { rows: 10, cols: 80 }
            }
        ).cols as usize;

        let disp_width = if self.fixed_width !=0 {
            self.fixed_width - 2
        } else {
            term_size - self.ext_padding.lr() - 2
        };

        // Parse box color only once per display
        let box_col_truecolor = match HexColor::parse(&self.box_col) {
            Ok(color) => Color::TrueColor { r: color.r, g: color.g, b: color.b },
            Err(e) => {
                eprintln!("Error parsing box color '{}': {}", &self.box_col, e);
                Color::White // Default color
            }
        };
        // Resolve template once per display
        let box_pieces = map_box_type(&self.type_enum);
        let horiz = box_pieces.horizontal.to_string().color(box_col_truecolor);
        
        let align_offset = align_offset(&disp_width, &term_size, &self.align, &self.ext_padding);

        // Printing the top segment
        print!("{:>width$}", box_pieces.top_left.to_string().color(box_col_truecolor), width=self.ext_padding.left+1+align_offset);
        for _ in 0..disp_width {
            print!("{}", horiz);
        }
        println!("{}", box_pieces.top_right.to_string().color(box_col_truecolor));

        // Iteratively print all the textbox sections, with appropriate dividers in between
        for i in 0..self.sect_count {
            if i > 0 {
                self.print_h_divider(&box_col_truecolor, disp_width, align_offset, &box_pieces);
            }
            self.display_segment(i, disp_width, align_offset, &box_pieces, &box_col_truecolor);
        }

        // Printing the bottom segment
        print!("{:>width$}", box_pieces.bottom_left.to_string().color(box_col_truecolor), width=self.ext_padding.left+1+align_offset);
        for _ in 0..disp_width {
            print!("{}", horiz);
        }
        println!("{}", box_pieces.bottom_right.to_string().color(box_col_truecolor));

    }

    fn print_cols(&self, seg_index: usize, term_size: usize, box_pieces: BoxTemplates, box_col_tc: Color::TrueColor) {
        let mut curr_line = 0;
        let mut data_counts : Vec<usize> = Vec::new();
        let mut width = term_size;
        loop {

            for i in 0..self.seg_cols_count[seg_index] {
                // print lines
                print!("{}", box_pieces.vertical.to_string().color(box_col_tc));
                display_col_segment_line(); //TODO: create this function, which takes a usize for last wordindex and resumes each segment from the last word on the next line
            }

            curr_line+=1;
            
        }
       
    }

    // Displaying each segment body
    fn display_segment(&mut self, seg_index: usize, disp_width: usize, align_offset: usize, box_pieces: &BoxTemplates, box_col_truecolor: &Color) {

        // TODO: Insert column printing branch here



        // Loop for all text lines
        for i in 0..self.data[seg_index].len() {
            // obtaining text colour truevalues
            let text_col_truecolor = match HexColor::parse(&self.colors[seg_index][i]) {
                Ok(color) => Color::TrueColor { r: color.r, g: color.g, b: color.b },
                Err(e) => {
                    eprintln!("Error parsing text color '{}': {}", &self.colors[seg_index][i], e);
                    Color::White // Default color
                }
            };
            // Processing data
            let processed_data = self.data[seg_index][i].trim().to_owned() + " ";

            let liner: Vec<String> = text_wrap_vec_fast(&processed_data, disp_width, &self.int_padding);

            // Generating new External Pad based on alignment offset
            let ext_offset = BoxPad {
                top: self.ext_padding.top,
                left: self.ext_padding.left + align_offset,
                right: self.ext_padding.right,
                down: self.ext_padding.down,
            }; 
            
            // Actually printing shiet

            // Iterative printing. Migrated from recursive to prevent stack overflows with larger text bodies and reduce complexity, also to improve code efficiency
            iter_line_prnt(&liner, box_pieces, box_col_truecolor, &text_col_truecolor, (&disp_width, &(self.fixed_width != 0)), (&ext_offset, &self.int_padding), &self.seg_align[seg_index]);

            // printing an empty line between consecutive non-terminal text line
            if i < self.data[seg_index].len() - 1 {
                println!("{1:>width$}{}{1}", " ".repeat(disp_width),
                         box_pieces
                             .vertical.to_string()
                             .color(*box_col_truecolor),
                         width=self.ext_padding.left+1+align_offset);
            }
        }
        // Recursive Printing of text -> now depreciated
        // recur_whitespace_printing(&processed_data, &mut ws_indices, &self.type_enum, &terminal_size, 0usize, &col_truevals, &self.ext_padding, &self.int_padding, &self.align);
    }

    // Printing the horizontal divider.
    fn print_h_divider(&self, box_col_truecolor: &Color, disp_width: usize, align_offset: usize, box_pieces: &BoxTemplates) {
        let horiz =  box_pieces.horizontal.to_string().color(*box_col_truecolor);
        print!("{:>width$}", box_pieces.left_t.to_string().color(*box_col_truecolor), width=self.ext_padding.left+1+align_offset);
        for _ in 0..disp_width {
            print!("{}", horiz);
        }
        println!("{}", box_pieces.right_t.to_string().color(*box_col_truecolor));
    }
}

// Faster non-allocating whitespace scanning text wrapper
fn text_wrap_vec_fast(data: &str, disp_width: usize, int_padding: &BoxPad) -> Vec<String> {
    let mut liner: Vec<String> = Vec::new();
    let max_len = disp_width.saturating_sub(int_padding.lr() + 2);
    if max_len == 0 { return liner; }
    let bytes = data.as_bytes();
    let mut start = 0usize;
    while start < data.len() {
        let mut end = (start + max_len).min(data.len());
        if end < data.len() {
            let mut last_space: Option<usize> = None;
            let mut j = start;
            while j < end {
                if bytes[j] == b' ' { last_space = Some(j); }
                j += 1;
            }
            if let Some(ws) = last_space { end = ws; }
        }
        liner.push(data[start..end].to_string());
        if end >= data.len().saturating_sub(1) { break; }
        // Advance past space if present to avoid leading spaces on next line
        start = if end < data.len() && bytes[end] == b' ' { end + 1 } else { end };
    }
    liner
}

fn iter_line_prnt(liner : &[String], box_pieces:&BoxTemplates, box_col: &Color, text_col: &Color, disp_params: (&usize, &bool), padding: (&BoxPad, &BoxPad), align: &BoxAlign) {
    let (ext_padding, int_padding) = padding;
    let (disp_width, fixed_size) = disp_params;
    let printable_area = disp_width - int_padding.lr() + 2*((int_padding.left!=0) as usize)*(!*fixed_size as usize); // IDK why this works, but it does
    let vertical = box_pieces.vertical.to_string().color(*box_col);
    match align {
        BoxAlign::Left => {
            for i in liner.iter() {
                print!("{:>width$}", vertical, width=ext_padding.left+1);
                print!("{:<pad$}", " ", pad=int_padding.left);
                print!("{:<width$}", i.color(*text_col), width=printable_area-(2*(!*fixed_size as usize))); // subtract 2 for the bars if on dynamic sizing
                print!("{:<pad$}", " ", pad=int_padding.right);
                println!("{}", vertical);
            }
        },
        BoxAlign::Center => {
            for i in liner.iter() {
                print!("{:>width$}", vertical, width=ext_padding.left+1);
                print!("{:<pad$}", " ", pad=int_padding.left+((printable_area-i.len())/2));
                print!("{}", i.color(*text_col));
                print!("{:<pad$}", " ", pad=int_padding.right+(printable_area-i.len())-((printable_area-i.len())/2));
                println!("{}", vertical);
            }
        },
        BoxAlign::Right => {
            for i in liner.iter() {
                print!("{:>width$}", vertical, width=ext_padding.left+1);
                print!("{:<pad$}", " ", pad=int_padding.left);
                print!("{:>width$}", i.color(*text_col), width=printable_area-(2*(!*fixed_size as usize))); // subtract 2 for the bars if on dynamic sizing
                print!("{:<pad$}", " ", pad=int_padding.right);
                println!("{}", vertical);
            }
        }
    }
}

// returns the box template for the given enum
fn map_box_type (boxtype : &BoxType) -> BoxTemplates{
    match boxtype {
        BoxType::Classic => CLASSIC_TEMPLATE,
        BoxType::Single => SINGLE_TEMPLATE,
        BoxType::DoubleHorizontal => DOUB_H_TEMPLATE,
        BoxType::DoubleVertical => DOUB_V_TEMPLATE,
        BoxType::Double => DOUBLE_TEMPLATE,
        BoxType::Bold => BOLD_TEMPLATE,
        BoxType::Rounded => ROUNDED_TEMPLATE,
        BoxType::BoldCorners => BOLD_CORNERS_TEMPLATE,
        BoxType::Empty => EMPTY_TEMPLATE,
    }
}

fn align_offset(disp_width: &usize, term_size: &usize, align: &BoxAlign, padding: &BoxPad) -> usize {
    match *align {
        BoxAlign::Left => {
            0
        },
        BoxAlign::Center => {
            (term_size-disp_width)/2 - padding.left
        },
        BoxAlign::Right => {
            term_size-(disp_width + 2*padding.right + padding.left)
        }
    }
}

// Macro type resolution fucntions for boxy!


// These helpers are public so the macro can access them across crate boundaries via $crate::boxer::...
// They are hidden from docs and not intended for direct user consumption.
#[doc(hidden)]
#[allow(dead_code)]
pub fn resolve_col(dat : String) -> String {
    dat
}
// Macro type-resolution function
#[doc(hidden)]
#[allow(dead_code)]
pub fn resolve_pad(dat : String) -> BoxPad {
    BoxPad::uniform(dat.parse::<usize>().unwrap_or(0usize))
}
// Macro type-resolution function
#[doc(hidden)]
#[allow(dead_code)]
pub fn resolve_align(dat : String) -> BoxAlign {
    match &*dat {
        "center" => BoxAlign::Center,
        "right" => BoxAlign::Right,
        "left" => BoxAlign::Left,
        _ => BoxAlign::Left,
    }
}
// Macro type-resolution function
#[doc(hidden)]
#[allow(dead_code)]
pub fn resolve_type(dat : String) -> BoxType{
    match &*dat {
        "classic" | "c" => BoxType::Classic,
        "single" | "s" => BoxType::Single,
        "double_horizontal" | "dh" => BoxType::DoubleHorizontal,
        "double_vertical" | "dv" => BoxType::DoubleVertical,
        "double" | "d" => BoxType::Double,
        "bold" | "b" => BoxType::Bold,
        "rounded" | "r" => BoxType::Rounded,
        "bold_corners" | "bc" => BoxType::BoldCorners,
        "empty" | "e" => BoxType::Empty,
        _ => BoxType::Single,
    }
}
// Macro type-resolution function
#[doc(hidden)]
#[allow(dead_code)]
pub fn resolve_segments(dat : String) -> usize {
    dat.parse().expect("failed to parse total segment number")
}

// Builder
/// The BoxyBuilder struct implements a fluent builder pattern for creating `Boxy` instances.
///
/// This builder provides a more expressive and readable way to create and configure text boxes.
/// Each method returns the builder instance itself, allowing method calls to be chained together.
/// When the configuration is complete, call the `build()` method to create the actual [`Boxy`](./struct.Boxy.html) instance.
///
/// # Examples
///
/// ```
/// use boxy_cli::prelude::*;
///
/// // Create and display a text box in a single fluent sequence
/// Boxy::builder()
///     .box_type(BoxType::Double)
///     .color("#00ffff")
///     .padding(BoxPad::uniform(1), BoxPad::from_tldr(2, 2, 1, 1))
///     .align(BoxAlign::Center)
///     .add_segment("Hello, Boxy!", "#ffffff", BoxAlign::Center)
///     .add_line("This is a new line.", "#32CD32")
///     .add_segment("Another section", "#663399", BoxAlign::Left)
///     .width(50)
///     .build()
///     .display();
/// ```
#[derive(Debug, Default)]
pub struct BoxyBuilder <'a> {
    type_enum: BoxType,
    data: Vec<Vec<Cow<'a, str>>>,
    box_col: String,
    colors: Vec<Vec<Cow<'a, str>>>,
    int_padding: BoxPad,
    ext_padding: BoxPad,
    align: BoxAlign,
    seg_align: Vec<BoxAlign>,
    fixed_width: usize,
    fixed_height: usize,
    seg_cols_ratio: Vec<Vec<usize>>,
    terminal_width_offset: i32,
}

impl <'a> BoxyBuilder <'a> {
    /// Creates a new `BoxyBuilder` with default values.
    ///
    /// This creates a builder with the following default values:
    /// - Box type: `BoxType::Single`
    /// - Color: empty string (will use white if not set)
    /// - Padding: zero on all sides
    /// - Alignment: `BoxAlign::Left`
    /// - No text segments
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let builder = BoxyBuilder::new();
    /// // Configure the builder with various methods
    /// let my_box = builder.box_type(BoxType::Double)
    ///                    .color("#00ffff")
    ///                    .build();
    /// ```
    ///
    /// Typically used through the `Boxy::builder()` factory method:
    ///
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let builder = Boxy::builder(); // Same as BoxyBuilder::new()
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the border type for the text box.
    ///
    /// This determines the visual style of the box borders, including the characters used for
    /// corners, edges, and intersections. Different styles can create different visual effects,
    /// from simple ASCII-style boxes to double-lined or rounded boxes.
    ///
    /// # Arguments
    ///
    /// * `box_type` - The border style from the [`BoxType`](../constructs/enum.BoxType.html) enum
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let builder = Boxy::builder()
    ///     .box_type(BoxType::Double); // Use double-lined borders
    ///
    /// // Or try other border styles
    /// let rounded_box = Boxy::builder()
    ///     .box_type(BoxType::Rounded)
    ///     .build();
    /// ```
    pub fn box_type(mut self, box_type: BoxType) -> Self {
        self.type_enum = box_type;
        self
    }
    
    /// Sets the border color for the text box.
    ///
    /// This method defines the color of the box borders, including corners, edges, and intersections.
    /// The color is specified using a hexadecimal color code (e.g. "#00ffff" for cyan).
    ///
    /// # Arguments
    ///
    /// * `box_color` - A hex color code string (e.g. "#00ffff", "#ff0000")
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// // Create a box with cyan borders
    /// let cyan_box = Boxy::builder()
    ///     .color("#00ffff")
    ///     .build();
    ///
    /// // Create a box with red borders
    /// let red_box = Boxy::builder()
    ///     .color("#ff0000")
    ///     .build();
    /// ```
    ///
    /// # Note
    ///
    /// The actual appearance depends on terminal support for colors.
    pub fn color(mut self, box_color: &str) -> Self {
        self.box_col = box_color.to_string();
        self
    }

    /// Adds a new text segment to the box with specified text, color, and alignment.
    ///
    /// Each segment represents a distinct section of the textbox that will be separated by
    /// horizontal dividers. This method is used to add the first or subsequent major
    /// sections of content.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content for this segment
    /// * `color` - A hex color code (e.g. "#ffffff") for the text color
    /// * `text_align` - The alignment for this text segment (left, center, right)
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let my_box = Boxy::builder()
    ///     // Add a centered header segment in white
    ///     .add_segment("Header", "#ffffff", BoxAlign::Center)
    ///     // Add a left-aligned content segment in green
    ///     .add_segment("Content goes here", "#00ff00", BoxAlign::Left)
    ///     .build();
    /// ```
    pub fn add_segment(mut self, text: &str, color: &str, text_align: BoxAlign) -> Self {
        self.data.push(vec![Cow::Owned(text.to_owned())]);
        self.colors.push(vec![Cow::Owned(color.to_owned())]);
        self.seg_align.push(text_align);
        self
    }

    /// Adds a new line of text to the most recently added segment.
    ///
    /// This method adds a line of text to the last segment that was created.
    /// The new line will appear below the existing content in that segment.
    /// Unlike `add_segment()`, this does not create a new segment with a divider.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content to add as a new line
    /// * `color` - A hex color code (e.g. "#ffffff") for the text color
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// let my_box = Boxy::builder()
    ///     // Add a header segment
    ///     .add_segment("Header", "#ffffff", BoxAlign::Center)
    ///     // Add a subheader as a new line in the same segment
    ///     .add_line("Subheader text", "#aaaaaa")
    ///     // Add a different segment with a divider
    ///     .add_segment("Content section", "#00ff00", BoxAlign::Left)
    ///     .build();
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if no segments have been added yet.
    pub fn add_line(mut self, text: &str, color: &str) -> Self {
        if let Some(last_segment) = self.data.last_mut() {
            last_segment.push(Cow::Owned(text.to_owned()));
        } else {
            self.data.push(vec![Cow::Owned(text.to_owned())]);
        }
        self.colors[self.data.len()-1].push(Cow::Owned(color.to_owned()));
        self
    }

    /// Sets the overall alignment of the text box within the terminal.
    ///
    /// This method controls the horizontal positioning of the entire text box relative to the 
    /// terminal width. It does not affect the alignment of text within the box segments,
    /// which is specified individually when adding segments.
    ///
    /// # Arguments
    ///
    /// * `alignment` - The alignment to use: `BoxAlign::Left`, `BoxAlign::Center`, or `BoxAlign::Right`
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// // Create a box centered in the terminal
    /// let centered_box = Boxy::builder()
    ///     .align(BoxAlign::Center)
    ///     .add_segment("This box is centered in the terminal", "#ffffff", BoxAlign::Left)
    ///     .build();
    ///
    /// // Create a box aligned to the right edge of the terminal
    /// let right_box = Boxy::builder()
    ///     .align(BoxAlign::Right)
    ///     .add_segment("This box is aligned to the right", "#ffffff", BoxAlign::Left)
    ///     .build();
    /// ```
    pub fn align(mut self, alignment: BoxAlign) -> Self {
        self.align = alignment;
        self
    }

    /// Sets the internal padding between the box border and its text content.
    ///
    /// Internal padding creates space between the border of the box and the text inside it,
    /// providing visual breathing room for the content.
    ///
    /// # Arguments
    ///
    /// * `padding` - A [`BoxPad`](../constructs/struct.BoxPad.html) instance specifying the internal padding values
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// // Set uniform internal padding of 2 spaces on all sides
    /// let padded_box = Boxy::builder()
    ///     .internal_padding(BoxPad::uniform(2))
    ///     .build();
    ///
    /// // Set different padding for each side (top, left, bottom, right)
    /// let custom_pad_box = Boxy::builder()
    ///     .internal_padding(BoxPad::from_tldr(1, 3, 1, 3))
    ///     .build();
    /// ```
    pub fn internal_padding(mut self, padding: BoxPad) -> Self {
        self.int_padding = padding;
        self
    }

    /// Sets the external padding between the terminal edges and the text box.
    ///
    /// External padding creates space between the edges of the terminal and the border of the box.
    /// This affects the positioning of the box within the terminal.
    ///
    /// # Arguments
    ///
    /// * `padding` - A [`BoxPad`](../constructs/struct.BoxPad.html) instance specifying the external padding values
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// // Add 5 spaces of external padding on all sides
    /// let padded_box = Boxy::builder()
    ///     .external_padding(BoxPad::uniform(5))
    ///     .build();
    ///
    /// // Add 10 spaces of padding on the left side only
    /// let left_padded_box = Boxy::builder()
    ///     .external_padding(BoxPad::from_tldr(0, 10, 0, 0))
    ///     .build();
    /// ```
    pub fn external_padding(mut self, padding: BoxPad) -> Self {
        self.ext_padding = padding;
        self
    }

    /// Sets both internal and external padding for the text box in a single call.
    ///
    /// This is a convenience method that combines setting both external padding (between terminal
    /// edges and box) and internal padding (between box border and text) in one call.
    ///
    /// # Arguments
    ///
    /// * `external` - A [`BoxPad`](../constructs/struct.BoxPad.html) instance for the external padding (between terminal edges and box)
    /// * `internal` - A [`BoxPad`](../constructs/struct.BoxPad.html) instance for the internal padding (between box border and text)
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// // Set both padding types at once
    /// let box_with_padding = Boxy::builder()
    ///     .padding(
    ///         BoxPad::from_tldr(1, 5, 1, 5),  // external padding
    ///         BoxPad::uniform(2)              // internal padding
    ///     )
    ///     .build();
    /// ```
    pub fn padding(mut self, external: BoxPad, internal: BoxPad) -> Self {
        self.ext_padding = external;
        self.int_padding = internal;
        self
    }

    /// Sets a fixed width for the text box instead of dynamically sizing it to the terminal width.
    ///
    /// By default, the text box automatically adjusts its width based on the terminal size.
    /// This method allows you to specify a fixed width instead, which can be useful for
    /// creating boxes of consistent size or for controlling the layout of multiple boxes.
    ///
    /// # Arguments
    ///
    /// * `width` - The desired width in number of characters (including borders)
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// // Create a box with a fixed width of 50 characters
    /// let fixed_width_box = Boxy::builder()
    ///     .width(50)
    ///     .add_segment("This box has a fixed width of 50 characters", "#ffffff", BoxAlign::Left)
    ///     .build();
    /// ```
    ///
    /// # Note
    ///
    /// Setting width to 0 returns to dynamic sizing based on terminal width.
    pub fn width(mut self, width: usize) -> Self {
        self.fixed_width = width;
        self
    }

    /// Sets a fixed height for the text box by adding whitespace above and below the text.
    ///
    ///
    /// # Note
    ///
    /// This feature is experimental and may not work as expected in the current version.
    /// Setting height to 0 returns to dynamic sizing based on content.
    ///
    ///
    /// This method allows you to specify a fixed height for the box, which can be useful for
    /// creating boxes of consistent size or for controlling the layout of multiple boxes.
    ///
    /// # Arguments
    ///
    /// * `height` - The desired height in number of lines (including borders)
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// // Create a box with a fixed height of 20 lines
    /// let fixed_height_box = Boxy::builder()
    ///     .height(20)
    ///     .add_segment("This box has a fixed height", "#ffffff", BoxAlign::Center)
    ///     .build();
    /// ```
    ///
    pub fn height(mut self, height: usize) -> Self {
        self.fixed_height = height;
        self
    }

    /// Sets the size ratios between segments for vertical divisions.
    ///
    ///
    /// # Note
    ///
    /// This feature is experimental and may not work as expected in the current version.
    /// Setting height to 0 returns to dynamic sizing based on content.
    ///
    ///
    /// This method allows you to specify the relative width ratios when dividing a segment
    /// vertically into columns. Each number in the `ratios` vector represents the relative
    /// width of a column.
    ///
    /// # Arguments
    ///
    /// * `seg_index` - The index of the segment to apply the ratios to
    /// * `ratios` - A vector of relative width values for each column
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// // Create a box with a segment that has three columns in a 1:2:1 ratio
    /// let columned_box = Boxy::builder()
    ///     .add_segment("Segment with columns", "#ffffff", BoxAlign::Center)
    ///     .segment_ratios(0, vec![1, 2, 1])
    ///     .build();
    /// ```
    ///
    pub fn segment_ratios(mut self, seg_index: usize, ratios: Vec<usize>) -> Self {
        if seg_index >= self.seg_cols_ratio.len() {
            self.seg_cols_ratio.resize(seg_index + 1, Vec::new());
        }
        self.seg_cols_ratio[seg_index] = ratios;
        self
    }

    /// Sets the offset used when calculating the dynamic width of the text box based on the terminal size.
    ///
    ///
    /// # Note
    ///
    /// This feature is experimental and may not work as expected in the current version.
    /// Setting height to 0 returns to dynamic sizing based on content.
    ///
    ///
    /// By default, when `fixed_width` is not set, the text box width is calculated as the terminal
    /// width minus 20 characters. This method allows you to customize this default offset to make
    /// the box wider or narrower relative to the terminal width.
    ///
    /// # Arguments
    ///
    /// * `offset` - The number of characters to subtract from the terminal width.
    ///   Positive values make the box narrower, negative values widen it.
    ///
    /// # Returns
    ///
    /// The builder instance for method chaining
    ///
    /// # Examples
    ///
    /// ```
    /// use boxy_cli::prelude::*;
    ///
    /// // Make the box 10 characters narrower than the default
    /// let narrower_box = Boxy::builder()
    ///     .set_terminal_width_offset(30) // terminal width - 30
    ///     .build();
    ///
    /// // Make the box 10 characters wider than the default
    /// let wider_box = Boxy::builder()
    ///     .set_terminal_width_offset(10) // terminal width - 10
    ///     .build();
    /// ```
    ///
    /// # Warning
    ///
    /// Using negative offsets can cause the box to extend beyond the terminal boundaries,
    /// which may result in unexpected display issues.
    ///
    pub fn set_terminal_width_offset(mut self, offset: i32) -> Self {
        self.terminal_width_offset = offset;
        self
    }


    /// Builds the `Boxy` instance.
    ///
    /// ```
    /// # use boxy_cli::prelude::*;
    /// # let mut my_box = BoxyBuilder::new();
    /// my_box.build();
    /// ```
    /// Subsequently, display using display()
    /// ```
    /// # use boxy_cli::prelude::*;
    /// # let mut my_box = BoxyBuilder::new();
    /// my_box.build().display();
    /// ```
    ///
    pub fn build(self) -> Boxy <'a> {
        Boxy {
            type_enum: self.type_enum,
            tot_seg: self.data.len(), 
            sect_count: self.data.len(),
            data: self.data,
            box_col: self.box_col,
            colors: self.colors,
            int_padding: self.int_padding,
            ext_padding: self.ext_padding,
            align: self.align,
            seg_align: self.seg_align,
            fixed_width: self.fixed_width,
            fixed_height: self.fixed_height,
            seg_cols_count: {
                let mut seg_cols_count = Vec::new();
                for seg in &self.seg_cols_ratio {
                    seg_cols_count.push(seg.len());
                }
                seg_cols_count
            },
            seg_cols_ratio: self.seg_cols_ratio,
            terminal_width_offset: self.terminal_width_offset,

        }
    }
}
