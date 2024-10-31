mod defs;

use crate::defs::*;

pub struct Boxy {
    data : Vec<String>,
    colors : Vec<String>,
    divy : Vec<usize>,

}

impl Boxy {
    pub fn new() -> Self {
        Boxy{
            data : Vec::<String>::new(),
            colors : Vec::<String>::new(),
            divy : Vec::<usize>::new(),
        }
    }

    pub fn add_line(&mut self, data_string : &str, color : &str) {
        self.data.push(String::from(data_string));
        self.colors.push(String::from(color));
    }
}

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
