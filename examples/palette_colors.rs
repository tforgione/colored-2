extern crate colored;

use colored::*;

fn main() {
    for i in 0 ..= 255 {
        print!("{}", format!("{:x}", i).on_palette(i));
        if i == 7 || i == 15 || (i >= 16 && i % 6 == 3) {
            println!();
        }
    }
}
