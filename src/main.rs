use ansi::{rgb, rgb_bg};

fn main() {
    println!("{}Hello, world!{}", rgb!(42, 255, 200), ansi::RESET);
    println!("{}Hello, world!{}", rgb_bg!(42, 255, 200), ansi::RESET);
}
