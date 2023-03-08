use ansi::{hex, rgb, rgb_bg};

fn main() {
    println!("{}Hello, world!{}", rgb!(42, 255, 200), ansi::RESET);
    println!("{}Hello, world!{}", rgb_bg!(42, 255, 200), ansi::RESET);
    dbg!(hex!("FF00FF"));
    dbg!(hex!("ff00ff"));
    dbg!(hex!(0xff00ff));
    dbg!(hex!(#ff00ff));
    // hex!(0xFF00FF, some);
    // hex!(#ff00f);
    // rgb!("hey", 0, 0);
    // rgb!(-1, 0, 0);
    // rgb!(256, 0, 0);
}
