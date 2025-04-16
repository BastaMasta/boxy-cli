use colored::{Color, Colorize};
use hex_color::HexColor;
use crate::templates::*;
use crate::constructs::*;


//TODO: Add Documentation and exaples for each method and/or segment. Especially the display method, the setters and the macro 


/// The main struct. contains all the data rekevant to the TextBox
#[derive(Debug)]
pub struct Boxy {
    pub type_enum: BoxType,
    pub data : Vec<Vec<String>>,
    pub sect_count: usize,
    pub box_col : String,
    pub colors : Vec<Vec<String>>,
    pub int_padding: BoxPad,
    pub ext_padding: BoxPad,
    pub align : BoxAlign,
    pub fixed_width: usize,
    pub fixed_height: usize,
    pub seg_v_div_count: Vec<usize>,
    pub seg_v_div_ratio: Vec<Vec<usize>>,
    pub tot_seg: usize,
}

// Default struct values for the textbox
impl Default for Boxy {
    fn default() -> Self {
        Self {
            type_enum: BoxType::Single,
            data : Vec::<Vec<String>>::new(),
            sect_count: 0usize,
            box_col: "#ffffff".to_string(),
            colors : Vec::<Vec<String>>::new(),
            int_padding: BoxPad::new(),
            ext_padding: BoxPad::new(),
            align : BoxAlign::Left,
            fixed_width: 0usize,
            fixed_height: 0usize,
            seg_v_div_count: Vec::<usize>::new(),
            seg_v_div_ratio: Vec::<Vec<usize>>::new(),
            tot_seg: 0usize,
        }
    }
}


impl Boxy {
    /// Retuns a new instance of the Boxy struct with specified border type and colour
    pub fn new(box_type: BoxType, box_color : &str) -> Self {
        Boxy {
            type_enum: box_type,
            box_col: box_color.to_string(),
            ..Self::default()
        }
    }
    pub fn builder() -> BoxyBuilder {
        BoxyBuilder::new()
    }

    /// Adds a new text segment/section to the textbox, separated by a horizontal divider.
    // Adding a new text segment/section to the textbox
    // also initializes the textbox with its first use -> adds main body text
    pub fn add_text_sgmt(&mut self, data_string : &str, color : &str) {
        self.data.push(vec![data_string.to_owned()]);
        self.colors.push(vec![String::from(color)]);
        self.sect_count+=1;
    }

    /// Adds a new text line to the segemnt with a specific index.
    // Adding a text line to a segemnt with a specific index
    pub fn add_text_line_indx(&mut self, data_string : &str, color: &str, seg_index : usize) {
        self.data[seg_index].push(data_string.to_owned());
        self.colors[seg_index].push(String::from(color));
    }
    
    /// Adds a new text line to the latest segment.
    // Adding a text line to the latest segment
    pub fn add_text_line(&mut self, data_string : &str, color : &str) {
        self.data[self.sect_count-1].push(String::from(data_string));
        self.colors[self.sect_count-1].push(String::from(color));
    }

    /// Sets the aligment of the text in the textbox.
    // Setting the Alignment maually
    pub fn set_align(&mut self, align: BoxAlign) {
        self.align = align;
    }

    /// Set the internal padding for the textbox. (Between border and text)
    ///
    /// !! provide a [`BoxPad`] Struct for the padding
    // Setting the Padding manually
    pub fn set_int_padding(&mut self, int_padding : BoxPad) {
        self.int_padding = int_padding;
    }
    /// Set the external padding for the textbox. (Between terminal limits and border)
    /// 
    /// !! provide a [`BoxPad`] Struct for the padding
    pub fn set_ext_padding(&mut self, ext_padding : BoxPad) {
        self.ext_padding = ext_padding;
    }
    /// Set the internal padding and external padding for the textbox.
    ///
    /// !! provide a [`BoxPad`] Struct for the padding
    pub fn set_padding(&mut self, ext_padding : BoxPad, int_padding : BoxPad) {
        self.int_padding = int_padding;
        self.ext_padding = ext_padding;
    }

    /// Sets a fixed width for the textbox insted of dynamically sizing it to the width of the terminal
    // Setting the Width manually
    pub fn set_width(&mut self, width : usize) {
        self.fixed_width = width;
    }

    /// Sets a fixed height for the textbox. (adds in whitespace above and below the text)
    ///
    /// !! This feature is a work in progress. it may not work with the current version of the crate
    // Setting the Height manually
    pub fn set_height(&mut self, height : usize) {
        self.fixed_height = height;
    }

    /// Sets the size-ratio between segments when using vertical divisions
    ///
    /// !! This feature is a work in progress. it may not work with the current version of the crate
    pub fn set_segment_ratios(&mut self, seg_index: usize, ratios: Vec<usize>) {
        if seg_index >= self.seg_v_div_ratio.len() {
            self.seg_v_div_ratio.resize(seg_index + 1, Vec::new());
        }
        self.seg_v_div_ratio[seg_index] = ratios;
    }

    /// Prints/Displays the textbox into the CLI
    // Main Display Function to display the textbox
    pub fn display(&mut self) {
        // Initialising Display Variables
        let disp_width = if self.fixed_width !=0 {
            self.fixed_width
        } else {
            let size = termsize::get();
            if let Some(terminal_size) = size {
                terminal_size.cols as usize - 20
            } else {
                return;
            }
        };

        let col_truevals = HexColor::parse(&self.box_col).unwrap();
        let box_pieces = map_box_type(&self.type_enum);
        let horiz =box_pieces.horizontal.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b);

        // Printing the top segment
        print!("{:>width$}", box_pieces.top_left.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b), width=self.ext_padding.left+1);
        for _ in 0..(disp_width -2*self.ext_padding.right) {
            print!("{}", horiz);
        }
        println!("{}", box_pieces.top_right.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));

        // Iteratively print all the textbox sections, with appropriate dividers in between
        for i in 0..self.sect_count {
            if i > 0 {
                self.print_h_divider(&*self.box_col.clone(), &disp_width);
            }
            self.display_segment(i, &disp_width);
        }

        // Printing bottom segment
        print!("{:>width$}", box_pieces.bottom_left.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b), width=self.ext_padding.left+1);
        for _ in 0..disp_width -2*self.ext_padding.right {
            print!("{}", horiz);
        }
        println!("{}", box_pieces.bottom_right.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));

    }

    // Displaying each individual segment body
    fn display_segment(&mut self, seg_index: usize, disp_width: &usize) {

        // TODO: Add functionality to create segments while displaying the textbox i.e. columns
        let box_color_result = HexColor::parse(&self.box_col);
        let box_col_truecolor = match box_color_result {
            Ok(color) => Color::TrueColor { r: color.r, g: color.g, b: color.b },
            Err(e) => {
                eprintln!("Error parsing box color '{}': {}", &self.box_col, e);
                Color::White // Default color
            }
        };

        // Loop for all text lines
        for i in 0..self.data[seg_index].len() {
            // obtaining text colour truevalues
            let text_color_result = HexColor::parse(&self.colors[seg_index][i]);
            let text_col_truecolor = match text_color_result {
                Ok(color) => Color::TrueColor { r: color.r, g: color.g, b: color.b },
                Err(e) => {
                    eprintln!("Error parsing text color '{}': {}", &self.colors[seg_index][i], e);
                    Color::White // Default color
                }
            };
            // Processing data
            let mut processed_data = String::with_capacity(self.data[seg_index][i].len()+1);
            processed_data.push_str(self.data[seg_index][i].trim());
            processed_data.push(' ');
            let mut ws_indices = Vec::new();
            // Creating a map of all whitespaces to help in text wrapping for this text segment\
            // looping over binary segments, as all other methods create a new iterator, taking up more mem
            let mut k = 0usize;
            while k < processed_data.len() {
                if processed_data.as_bytes()[k] == b' ' {
                    ws_indices.push(k);
                }
                k += 1;
            }

            let liner: Vec<String> = text_wrap_vec(&processed_data, &mut ws_indices, &disp_width.clone(), &self.ext_padding, &self.int_padding);

            // Actually printing shiet

            // Iterative printing. migrated form recursive to prevent stack overflows and reduce complexity, also to improve code efficiency
            iter_line_prnt(&liner, map_box_type(&self.type_enum), &box_col_truecolor, &text_col_truecolor,disp_width, &self.ext_padding, &self.int_padding, &self.align);

            // printing an empty line between consecutive non-terminal text line
            if i < self.data[seg_index].len() - 1 {
                println!("{1:>width$}{}{1}", " ".repeat(disp_width - self.ext_padding.lr()),
                         map_box_type(&self.type_enum)
                             .vertical.to_string()
                             .color(box_col_truecolor),
                         width=self.ext_padding.left+1);
            }
        }
        // Recursive Printing of text -> now depreciated
        // recur_whitespace_printing(&processed_data, &mut ws_indices, &self.type_enum, &terminal_size, 0usize, &col_truevals, &self.ext_padding, &self.int_padding, &self.align);
    }

    // Printing the horizontal divider.
    fn print_h_divider(&mut self, boxcol_hex: &str, disp_width: &usize){
        let box_pieces = map_box_type(&self.type_enum);
        let box_col_truecolor = match HexColor::parse(boxcol_hex) {
            Ok(color) => Color::TrueColor { r: color.r, g: color.g, b: color.b },
            Err(e) => {
                eprintln!("Error parsing divider color '{}': {}", boxcol_hex, e);
                Color::White // Default color
            }
        };
        let horiz =  box_pieces.horizontal.to_string().color(box_col_truecolor);
        print!("{:>width$}", box_pieces.left_t.to_string().color(box_col_truecolor), width=self.ext_padding.left+1);
        for _ in 0..*disp_width-self.ext_padding.lr() {
            print!("{}", horiz);
        }
        println!("{}", box_pieces.right_t.to_string().color(box_col_truecolor));
    }

    //TODO: Set up the boxy struct to have a vec for each segment, with a cons list for each individual segment
    // that'll make sure that all segments have a proper implementation, and no segments have mismatched data datatypes
    
    //TODO: have the function fetch the data for it's sub-segment only, and not repeat the same for all the segments

    //TODO: Resolve the internal whiitespace padding issue which occurs when longer segments are placed beside shorter segments
    //TODO: check if the whitespace padidng is uniform along the length, if not squish the bug.
    
    //TODO: Not kill yourself while doing this

    // Display a segment divided into mini-segments based on ratios
    fn _display_segment_with_ratios(&mut self, seg_index: usize, terminal_size: &usize) {
        let col_truevals = HexColor::parse(&self.box_col).unwrap();
        let box_pieces = map_box_type(&self.type_enum);

        // Fetch ratios for the segment
        let ratios = if seg_index < self.seg_v_div_ratio.len() {
            &self.seg_v_div_ratio[seg_index]
        } else {
            static EMPTY_VEC: Vec<usize> = Vec::new();
            &EMPTY_VEC
        };

        if ratios.is_empty() {
            // If no ratios are defined, fallback to a single segment
            self.display_segment(seg_index, terminal_size);
            return;
        }

        // Calculate total ratio and widths for each mini-segment
        let total_ratio: usize = ratios.iter().sum();
        let printable_width = terminal_size - self.ext_padding.lr();
        let segment_widths: Vec<usize> = ratios.iter().map(|r| r * printable_width / total_ratio).collect();

        // Prepare text for each mini-segment
        let mut mini_segments: Vec<Vec<String>> = vec![Vec::new(); ratios.len()];
        let lines = &self.data[seg_index];
        for line in lines.iter() {
            let mut processed_data = String::with_capacity(line.len() + 1);
            processed_data.push_str(line.trim());
            processed_data.push(' ');

            let mut ws_indices = Vec::new();
            let mut k = 0usize;
            while k < processed_data.len() {
                if processed_data.as_bytes()[k] == b' ' {
                    ws_indices.push(k);
                }
                k += 1;
            }

            // Distribute text into mini-segments
            for (j, width) in segment_widths.iter().enumerate() {
                let liner = text_wrap_vec(&processed_data, &mut ws_indices, width, &self.ext_padding, &self.int_padding);
                mini_segments[j].extend(liner);
            }
        }

        // Print each line of the mini-segments with vertical dividers
        let max_lines = mini_segments.iter().map(|seg| seg.len()).max().unwrap_or(0);
        for line_index in 0..max_lines {
            // Print the left padding and vertical bar
            print!("{:>width$}", box_pieces.vertical.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b), width = self.ext_padding.left + 1);

            for (j, segment) in mini_segments.iter().enumerate() {
                if line_index < segment.len() {
                    // Print the text in the mini-segment
                    print!("{:<pad$}", " ", pad = self.int_padding.left);
                    print!("{:<width$}", segment[line_index], width = segment_widths[j] - self.int_padding.lr());
                    print!("{:<pad$}", " ", pad = self.int_padding.right);
                } else {
                    // Print empty space if no text exists for this line
                    print!("{:<width$}", " ", width = segment_widths[j]);
                }

                // Print vertical divider between mini-segments
                if j < mini_segments.len() - 1 {
                    print!("{}", box_pieces.vertical.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
                }
            }

            // Print the right padding and vertical bar
            println!("{}", box_pieces.vertical.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
        }
    }
}

// Function to find the next-most-fitting string slice for the give terminal size

fn nearest_whitespace(map: &mut Vec<usize>, printable_length: &usize, start_index: usize) -> usize {
    let mut next_ws = 0;
    for i in map {
        if *i > start_index && *i-start_index <= *printable_length {
            next_ws = *i;
        }
    }
    // force line break if no appropriate whitespace found
    if next_ws == 0 {
        next_ws = start_index + printable_length;
    }
    next_ws
}

// Recursively printing the next text segment into the textbox

// Went with recursive as that is just more modular, and I can just reuse this code for printing horizontal and vertical segments.

fn text_wrap_vec(data:&str, map: &mut Vec<usize>, disp_width: &usize, ext_padding: &BoxPad, int_padding: &BoxPad) -> Vec<String> {
    let mut liner: Vec<String> = Vec::new();
    let mut start_index = 0;

    while start_index < data.len() {
        let next_ws = nearest_whitespace(map, &(disp_width - (int_padding.lr() + ext_padding.lr()) - 2), start_index);
        liner.push(data[start_index..next_ws].to_string());
        if next_ws >= data.len()-1 {break;}
        start_index = next_ws+1;
    }
    liner

    // Legacy recursive code. Depreciated to increase efficiency for larger use cases
    /*
    let next_ws = nearest_whitespace(map, &(term_size - 2*(int_padding + ext_padding)), start_index);
    line_vec.push(String::from(&data[start_index..next_ws]));
    if next_ws < (data.len()-1) {
        text_wrap_vec(data, map, term_size, next_ws+1, ext_padding, int_padding, line_vec);
    }
    */
}


fn iter_line_prnt(liner : &[String], box_pieces:BoxTemplates, box_col: &Color, text_col: &Color, disp_width: &usize, ext_padding: &BoxPad, int_padding: &BoxPad, align: &BoxAlign) {
    let printable_area = disp_width - (int_padding.lr() + ext_padding.lr());
    let vertical = box_pieces.vertical.to_string().color(*box_col);
    match align {
        BoxAlign::Left => {
            for i in liner.iter() {
                print!("{:>width$}", vertical, width=ext_padding.left+1);
                print!("{:<pad$}", " ", pad=int_padding.left);
                print!("{:<width$}", i.color(*text_col), width=printable_area-2); // subtract 2 for the bars
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
                print!("{:>width$}", i.color(*text_col), width=printable_area-2); // subtract 2 for the bars
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
    }
}

// Macro type resolution fucntions for boxy!


/// Macro type-resolution function
pub fn resolve_col(dat : String) -> String {
    dat
}
/// Macro type-resolution function
pub fn resolve_pad(dat : String) -> BoxPad {
    BoxPad::uniform(dat.parse::<usize>().unwrap_or(0usize))
}
/// Macro type-resolution function
pub fn resolve_align(dat : String) -> BoxAlign {
    match &*dat {
        "center" => BoxAlign::Center,
        "right" => BoxAlign::Right,
        "left" => BoxAlign::Left,
        _ => BoxAlign::Left,
    }
}
/// Macro type-resolution function
pub fn resolve_type(dat : String) -> BoxType{
    match &*dat {
        "classic" => BoxType::Classic,
        "single" => BoxType::Single,
        "double_horizontal" => BoxType::DoubleHorizontal,
        "double_vertical" => BoxType::DoubleVertical,
        "double" => BoxType::Double,
        "bold" => BoxType::Bold,
        "rounded" => BoxType::Rounded,
        "bold_corners" => BoxType::BoldCorners,
        _ => BoxType::Single,
    }
}
/// Macro type-resolution function
pub fn resolve_segments(dat : String) -> usize {
    dat.parse().expect("failed to parse total segment number")
}


// Builder
#[derive(Debug, Default)]
pub struct BoxyBuilder {
    type_enum: BoxType,
    data: Vec<Vec<String>>,
    box_col: String,
    colors: Vec<Vec<String>>,
    int_padding: BoxPad,
    ext_padding: BoxPad,
    align: BoxAlign,
    fixed_width: usize,
    fixed_height: usize,
    seg_v_div_count: Vec<usize>,
    seg_v_div_ratio: Vec<Vec<usize>>,
}

impl BoxyBuilder {
    /// Creates a new `BoxyBuilder` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the border type for the `Boxy` instance.
    pub fn box_type(mut self, box_type: BoxType) -> Self {
        self.type_enum = box_type;
        self
    }

    /// Sets the border color for the `Boxy` instance.
    pub fn color(mut self, box_color: &str) -> Self {
        self.box_col = box_color.to_string();
        self
    }

    /// Adds a new text segment with its color.
    pub fn add_segment(mut self, text: &str, color: &str) -> Self {
        self.data.push(vec![text.to_owned()]);
        self.colors.push(vec![color.to_owned()]);
        self
    }

    /// Adds a new text line to the last added segment with its color.
    pub fn add_line(mut self, text: &str, color: &str) -> Self {
        if let Some(last_segment) = self.data.last_mut() {
            last_segment.push(text.to_owned());
        } else {
            self.data.push(vec![text.to_owned()]);
        }
        self.colors[self.data.len()-1].push(color.to_owned());
        self
    }

    /// Sets the text alignment for the `Boxy` instance.
    pub fn align(mut self, alignment: BoxAlign) -> Self {
        self.align = alignment;
        self
    }

    /// Sets the internal padding for the `Boxy` instance.
    pub fn internal_padding(mut self, padding: BoxPad) -> Self {
        self.int_padding = padding;
        self
    }

    /// Sets the external padding for the `Boxy` instance.
    pub fn external_padding(mut self, padding: BoxPad) -> Self {
        self.ext_padding = padding;
        self
    }

    /// Sets both internal and external padding.
    pub fn padding(mut self, external: BoxPad, internal: BoxPad) -> Self {
        self.ext_padding = external;
        self.int_padding = internal;
        self
    }

    /// Sets a fixed width for the `Boxy` instance.
    pub fn width(mut self, width: usize) -> Self {
        self.fixed_width = width;
        self
    }

    /// Sets a fixed height for the `Boxy` instance.
    pub fn height(mut self, height: usize) -> Self {
        self.fixed_height = height;
        self
    }

    /// Sets the size ratios between segments for vertical divisions.
    pub fn segment_ratios(mut self, seg_index: usize, ratios: Vec<usize>) -> Self {
        if seg_index >= self.seg_v_div_ratio.len() {
            self.seg_v_div_ratio.resize(seg_index + 1, Vec::new());
        }
        self.seg_v_div_ratio[seg_index] = ratios;
        self
    }

    /// Builds the `Boxy` instance.
    pub fn build(self) -> Boxy {
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
            fixed_width: self.fixed_width,
            fixed_height: self.fixed_height,
            seg_v_div_count: Vec::new(), // These are calculated later if needed
            seg_v_div_ratio: self.seg_v_div_ratio,

        }
    }
}