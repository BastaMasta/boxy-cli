use std::fmt::Display;
use colored::Colorize;
use hex_color::HexColor;
use crate::templates::*;


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



#[derive(Debug)]
pub struct Boxy {
    pub type_enum: BoxType,
    pub data : Vec<String>,
    pub sect_count: usize,
    pub box_col : String,
    pub colors : Vec<String>,
    pub int_padding: usize,
    pub ext_padding: usize,
    pub align : BoxAlign,

}

// Default struct values for the textbox
impl Default for Boxy {
    fn default() -> Self {
        Self {
            type_enum: BoxType::Single,
            data : Vec::<String>::new(),
            sect_count: 0usize,
            box_col: "#ffffff".to_string(),
            colors : Vec::<String>::new(),
            int_padding: 5usize,
            ext_padding: 5usize,
            align : BoxAlign::Left,
        }
    }
}


// boxy macro
#[macro_export]
macro_rules! boxy {
    ($($key:ident: $value:expr),* $(,)?) => {{
        let mut boxy = Boxy::default();
        $(
            match stringify!($key) {
                "type" => boxy.type_enum = resolve_type($value.to_string()),
                "color" => boxy.box_col = resolve_col($value.to_string()),
                "internal_pad" => boxy.int_padding = resolve_pad($value.to_string()),
                "external_pad" => boxy.ext_padding = resolve_pad($value.to_string()),
                "alignment" => boxy.align = resolve_align($value.to_string()),
                _ => panic!("Unknown field: {}", stringify!($key)),
            }
        )*
        boxy
    }};
}

impl Boxy {
    pub fn new(box_type: BoxType, box_color : &str) -> Self {
        Boxy{
            type_enum: box_type,
            data : Vec::<String>::new(),
            sect_count: 0usize,
            box_col : (&box_color).to_string(),
            colors : Vec::<String>::new(),
            int_padding: 5usize,
            ext_padding: 5usize,
            align : BoxAlign::Left,
        }
    }

    // Adding a new test segment/section to the textbox
    // also initializes the textbox with its first use -> adds main body text
    pub fn add_text_sgmt(&mut self, data_string : &str, color : &str) {
        self.data.push(String::from(data_string));
        self.colors.push(String::from(color));
        self.sect_count+=1;
    }

    // Setting the Alignment maually
    pub fn set_align(&mut self, align: BoxAlign) {
        self.align = align;
    }

    // Setting the Padding manually
    pub fn set_padding(&mut self, ext_padding : usize, int_padding : usize) {
        self.ext_padding = ext_padding;
        self.int_padding = int_padding;
    }

   // Main Display Function to display the textbox
    pub fn display(&mut self) {
        // Initialising Display Variables
        let term = termsize::get().unwrap();
        let terminal_size = (term.cols as usize) - 20 - 2*self.ext_padding;
        let col_truevals = HexColor::parse(&self.box_col).unwrap();
        let box_pieces = map_box_type(&self.type_enum);
        
        // Printing the top segment
        print!("{:>width$}", box_pieces.top_left.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b), width=self.ext_padding+1);
        for _ in 0..terminal_size {
            print!("{}", box_pieces.horizontal.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
        }
        println!("{}", box_pieces.top_right.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));

        // Iteratively print all the textbox sections, with appropriate dividers in between
        for i in 0..self.sect_count {
            if i > 0 {
                self.print_h_divider(&col_truevals,  &terminal_size);
            }
            match self.align {
                BoxAlign::Left => {
                    self.display_segment(i, &(&terminal_size));
                }
                BoxAlign::Center => {
                    self.display_segment(i, &(&terminal_size - self.int_padding));
                }
                BoxAlign::Right => {
                    self.display_segment(i, &(&terminal_size - self.int_padding));
                }
            }
        }

        // Printing bottom segment
        print!("{:>width$}", box_pieces.bottom_left.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b), width=self.ext_padding+1);
        for _ in 0..terminal_size {
            print!("{}", box_pieces.horizontal.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
        }
        println!("{}", box_pieces.bottom_right.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));


    }
    fn display_segment(&mut self, seg_index: usize, terminal_size: &usize) {
        // TODO: Add functionality to create segments while displaying the textbox i.e. columns
        let col_truevals = HexColor::parse(&self.box_col).unwrap();
 
        // Processing data ad setting up whitespaces map
        let mut processed_data = String::from (self.data[seg_index].trim());
        processed_data.push(' ');
        let whitespace_indices_temp = processed_data.match_indices(" ").collect::<Vec<_>>();
        let mut ws_indices = Vec::new();
        for (i,_) in whitespace_indices_temp {
            ws_indices.push(i);
        }

        // Actually printing shiet

        // Recursive Printing of text
        recur_whitespace_printing(&processed_data, &mut ws_indices, &self.type_enum, &(terminal_size-self.int_padding), 0usize, &col_truevals, &self.ext_padding, &self.int_padding, &self.align);

    }


    // Printing the horizontal divider.

    fn print_h_divider(&mut self, boxcol: &HexColor, term_size: &usize){
        let box_pieces = map_box_type(&self.type_enum);
        print!("{:>width$}", box_pieces.left_t.to_string().truecolor(boxcol.r, boxcol.g, boxcol.b), width=self.ext_padding+1);
            for _ in 0..*term_size {
                print!("{}", box_pieces.horizontal.to_string().truecolor(boxcol.r, boxcol.g, boxcol.b));
            }
            println!("{}", box_pieces.right_t.to_string().truecolor(boxcol.r, boxcol.g, boxcol.b));
    }


    // TODO: Add vertical divider printing functionality

    // fn print_v_divider() {
    //
    // }

}


// Function to find the next-most-fitting string slice for the give terminal size

fn nearest_whitespace(map: &mut Vec<usize>, term_size: &usize, start_index: usize) -> usize {
    let mut next_ws = 0;
    for i in map {
        if *i > start_index && *i-start_index <= *term_size {
            next_ws = *i;
        }
    }
    next_ws
}

// Recursively printing the next text segment into the textbox

// Went with recursive as that is just more modular, and I can just reuse this code for printing horizontal and vertical segments.



fn recur_whitespace_printing(data:&str, map: &mut Vec<usize>, boxtype: &BoxType, term_size: &usize, start_index: usize, boxcol: &HexColor, ext_padding: &usize, int_padding: &usize, align : &BoxAlign) {
    let box_pieces = map_box_type(boxtype);
    print!("{:>width$}", box_pieces.vertical.to_string().truecolor(boxcol.r, boxcol.g, boxcol.b), width=*ext_padding+1);
    let next_ws = nearest_whitespace(map, &(term_size - int_padding), start_index);

    match align {
        BoxAlign::Left => {
            print!("{:<pad$}", " ", pad=*int_padding);
            print!("{:<width$}", &data[start_index..next_ws], width=term_size,);
        }
        BoxAlign::Center => {
            print!("{:<pad$}", " ", pad=*int_padding + ((term_size-(next_ws-start_index))/2));
            print!("{:<width$}", &data[start_index..next_ws], width=term_size-((term_size-(next_ws-start_index))/2)+*int_padding);
        }
        _ => {}
    }
    print!("{}", box_pieces.vertical.to_string().truecolor(boxcol.r, boxcol.g, boxcol.b));
    println!(" ");
    if next_ws < (data.len()-1) {
        recur_whitespace_printing(data, map, boxtype, term_size, next_ws+1, boxcol, ext_padding, int_padding, align);
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
    }
}

// Macro Resolution fucntions for boxy!

pub fn resolve_col(dat : String) -> String {
    return dat
}
pub fn resolve_pad(dat : String) -> usize {
    return dat.parse::<usize>().unwrap_or(0usize);
}
pub fn resolve_align(dat : String) -> BoxAlign {
    return match &*dat {
        "center" => BoxAlign::Center,
        "right" => BoxAlign::Right,
        "left" => BoxAlign::Left,
        _ => BoxAlign::Left,
    }
}pub fn resolve_type(dat : String) -> BoxType{
    return match &*dat {
        "classic" => BoxType::Classic,
        "single" => BoxType::Single,
        "double_horizontal" => BoxType::DoubleHorizontal,
        "double_vertical" => BoxType::DoubleVertical,
        "double" => BoxType::Double,
        "bold" => BoxType::Bold,
        "rounded" => BoxType::Rounded,
        _ => BoxType::Single,
    }
}



// Jargon function, purely for testing

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
