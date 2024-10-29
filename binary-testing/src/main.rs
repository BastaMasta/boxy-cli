use colored::Colorize;

fn main() {
    boxy();
}


fn boxy() {
    let light = ["┌", "┐", "└", "┘", "│", "─", "├", "┼", "┤"];
    print!("{}", light[0].blue());
    for _ in 0..50 {
        print!("{}", light[5].blue());
    }
    println!("{}", light[1].blue());
    for __ in 0..5 {
        print!("{}", light[4].blue());
        for _ in 0..50 {
            print!(" ") 
        };
        print!("{}", light[4].blue());
        println!("");
    }
    print!("{}", light[4].blue());
    print!("12345678901234567890123456789012345678901234567890");
    print!("{}", light[4].blue());
    println!("");

}