use ansi::{hex, rgb, rgb_bg};

fn main() {
    println!("{}Hello, world!{}", rgb!(42, 255, 200), ansi::RESET);
    println!("{}Hello, world!{}", rgb_bg!(42, 255, 200), ansi::RESET);
    dbg!(hex!("ff00ff"));
    println!("{}", 0xff00ff);
    println!("{}", 0xFF00FF);
    dbg!(hex!(0xff00ff));
    dbg!(hex!(0xFF00FF, some));
}
