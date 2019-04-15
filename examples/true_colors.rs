extern crate colored;

use colored::*;

fn main() {
    for i in (0 .. 255).step_by(15) {
        for j in (0 .. 255).step_by(5) {
            print!("{}", " ".on_true_color(i, j, 255));
        }
        println!();
    }
}
