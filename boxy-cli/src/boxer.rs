use colored::Colorize;
use hex_color::HexColor;
use crate::templates::*;
use crate::constructs::*;

#[derive(Debug)]
pub struct Boxy {
    pub type_enum: BoxType,
    pub data : Vec<Vec<String>>,
    pub sect_count: usize,
    pub box_col : String,
    pub colors : Vec<String>,
    pub int_padding: usize,
    pub ext_padding: usize,
    pub align : BoxAlign,
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
            colors : Vec::<String>::new(),
            int_padding: 5usize,
            ext_padding: 5usize,
            align : BoxAlign::Left,
            tot_seg: 0usize,
        }
    }
}


impl Boxy {
    pub fn new(box_type: BoxType, box_color : &str) -> Self {
        Boxy{
            type_enum: box_type,
            data : Vec::<Vec<String>>::new(),
            sect_count: 0usize,
            box_col : (&box_color).to_string(),
            colors : Vec::<String>::new(),
            int_padding: 5usize,
            ext_padding: 5usize,
            align : BoxAlign::Left,
            tot_seg: 0usize,
        }
    }

    // Adding a new test segment/section to the textbox
    // also initializes the textbox with its first use -> adds main body text
    pub fn add_text_sgmt(&mut self, data_string : &str, color : &str) {
        self.data.push(vec![data_string.to_owned()]);
        self.colors.push(String::from(color));
        self.sect_count+=1;
    }

    // Adding a text line to a segemnt with a specific index
    pub fn add_text_line_indx(&mut self, data_string : &str, seg_index : usize) {
        self.data[seg_index].push(data_string.to_owned());
    }
    
    // Adding a text line to the latest segment
    pub fn add_text_line(&mut self, data_string : &str) {
        self.data[self.sect_count-1].push(String::from(data_string));
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
        let terminal_size = (term.cols as usize) - 20;
        let col_truevals = HexColor::parse(&self.box_col).unwrap();
        let box_pieces = map_box_type(&self.type_enum);
        
        // Printing the top segment
        print!("{:>width$}", box_pieces.top_left.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b), width=self.ext_padding+1);
        for _ in 0..(terminal_size-2*self.ext_padding) {
            print!("{}", box_pieces.horizontal.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
        }
        println!("{}", box_pieces.top_right.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));

        // Iteratively print all the textbox sections, with appropriate dividers in between
        for i in 0..self.sect_count {
            if i > 0 {
                self.print_h_divider(&col_truevals,  &terminal_size);
            }
            self.display_segment(i, &terminal_size);

        }

        // Printing bottom segment
        print!("{:>width$}", box_pieces.bottom_left.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b), width=self.ext_padding+1);
        for _ in 0..terminal_size-2*self.ext_padding {
            print!("{}", box_pieces.horizontal.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));
        }
        println!("{}", box_pieces.bottom_right.to_string().truecolor(col_truevals.r, col_truevals.g, col_truevals.b));


    }
    
    // Displaying each segment body
    fn display_segment(&mut self, seg_index: usize, terminal_size: &usize) {

        // TODO: Add functionality to create segments while displaying the textbox i.e. columns
        let col_truevals = HexColor::parse(&self.box_col).unwrap();

        // Loop for all text lines
        for i in 0..self.data[seg_index].len() {
            // Processing data
            let mut processed_data = String::with_capacity(self.data[seg_index][i].len()+1);
            processed_data.push_str(self.data[seg_index][i].trim());
            processed_data.push(' ');
            let mut ws_indices = Vec::new();
            
            // Whitespace Map
            let mut k = 0usize;
            while k < processed_data.len() {
                if processed_data.as_bytes()[k] == b' ' {
                    ws_indices.push(k);
                }
                k += 1;
            }

            let liner: Vec<String> = text_wrap_vec(&processed_data, &mut ws_indices, &terminal_size.clone(), &self.ext_padding, &self.int_padding);

            // Actually printing shiet

            // Iterative printing. migrated form recursive to prevent stack overflows and reduce complexity, also to improve code efficiency
            iter_line_prnt(&liner, map_box_type(&self.type_enum), &col_truevals, terminal_size, &self.ext_padding, &self.int_padding, &self.align);

            // printing an empty line between consecutive non-terminal text line
            if i < self.data[seg_index].len() - 1 {
                println!("{1:>width$}{}{1}", " ".repeat(terminal_size - 2 * self.ext_padding),
                         map_box_type(&self.type_enum)
                             .vertical.to_string()
                             .truecolor(col_truevals.r, col_truevals.g, col_truevals.b),
                         width=self.ext_padding+1);
            }
        }
        // Recursive Printing of text -> now depreciated
        // recur_whitespace_printing(&processed_data, &mut ws_indices, &self.type_enum, &terminal_size, 0usize, &col_truevals, &self.ext_padding, &self.int_padding, &self.align);
    }

    // Printing the horizontal divider.
    fn print_h_divider(&mut self, boxcol: &HexColor, term_size: &usize){
        let box_pieces = map_box_type(&self.type_enum);
        print!("{:>width$}", box_pieces.left_t.to_string().truecolor(boxcol.r, boxcol.g, boxcol.b), width=self.ext_padding+1);
            for _ in 0..*term_size-2*self.ext_padding {
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

fn text_wrap_vec(data:&str, map: &mut Vec<usize>, term_size: &usize, ext_padding: &usize, int_padding: &usize) -> Vec<String> {
    let mut liner: Vec<String> = Vec::new();
    let mut start_index = 0;

    while start_index < data.len() {
        let next_ws = nearest_whitespace(map, &(term_size - 2*(int_padding + ext_padding)), start_index);
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


fn iter_line_prnt(liner : &[String], box_pieces:BoxTemplates, box_col: &HexColor, term_size: &usize, ext_padding: &usize, int_padding: &usize, align: &BoxAlign) {
    let printable_area = term_size-2*(ext_padding+int_padding);
    match align {
        BoxAlign::Left => {
            for i in liner.iter() {
                print!("{:>width$}", box_pieces.vertical.to_string().truecolor(box_col.r, box_col.g, box_col.b), width=*ext_padding+1);
                print!("{:<pad$}", " ", pad=*int_padding);
                print!("{:<width$}", i, width=printable_area);
                print!("{:<pad$}", " ", pad=*int_padding);
                println!("{}", box_pieces.vertical.to_string().truecolor(box_col.r, box_col.g, box_col.b));
            }
        },
        BoxAlign::Center => {
            for i in liner.iter() {
                print!("{:>width$}", box_pieces.vertical.to_string().truecolor(box_col.r, box_col.g, box_col.b), width=*ext_padding+1);
                print!("{:<pad$}", " ", pad=*int_padding+((printable_area-i.len())/2));
                print!("{}", i);
                print!("{:<pad$}", " ", pad=*int_padding+(printable_area-i.len())-((printable_area-i.len())/2));
                println!("{}", box_pieces.vertical.to_string().truecolor(box_col.r, box_col.g, box_col.b));
            }
        },
        BoxAlign::Right => {
            for i in liner.iter() {
                print!("{:>width$}", box_pieces.vertical.to_string().truecolor(box_col.r, box_col.g, box_col.b), width=*ext_padding+1);
                print!("{:<pad$}", " ", pad=*int_padding);
                print!("{:>width$}", i, width=printable_area);
                print!("{:<pad$}", " ", pad=*int_padding);
                println!("{}", box_pieces.vertical.to_string().truecolor(box_col.r, box_col.g, box_col.b));
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

pub fn resolve_col(dat : String) -> String {
    dat
}
pub fn resolve_pad(dat : String) -> usize {
    dat.parse::<usize>().unwrap_or(0usize)
}
pub fn resolve_align(dat : String) -> BoxAlign {
    match &*dat {
        "center" => BoxAlign::Center,
        "right" => BoxAlign::Right,
        "left" => BoxAlign::Left,
        _ => BoxAlign::Left,
    }
}
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
pub fn resolve_segments(dat : String) -> usize {
    dat.parse().expect("failed to parse total segment number")
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