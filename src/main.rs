use ansi::rgb;

fn main() {
    println!("{}Hello, world!{}", rgb!(42, 255, 0), ansi::RESET);
}
