#[cfg(feature = "ansi")]
pub use ansi;

#[cfg(feature = "hmerr")]
pub use hmerr;

#[cfg(feature = "ux")]
pub use ux;

#[cfg(feature = "spof")]
pub use spof;

#[cfg(test)]
mod tests {

    #[test]
    fn test_ansi() {
        use ansi::{hex, rgb, rgb_bg};

        const SOME: &str = rgb!(128, 0, 42);

        println!("{}Hello, world!{}", rgb!(42, 255, 200), ansi::RESET);
        println!("{}Hello, world!{}", rgb_bg!(42, 255, 200), ansi::RESET);
        dbg!(hex!("FF00FF"));
        dbg!(hex!("ff00ff"));
        dbg!(hex!(0xff00ff));
        dbg!(hex!(#ff00ff));
        println!("{}Hello, world!{}", SOME, ansi::RESET);
        // hex!(0xFF00FF, some);
        // hex!(#ff00f);
        // rgb!("hey", 0, 0);
        // rgb!(-1, 0, 0);
        // rgb!(256, 0, 0);
    }

    /*
    #[test]
    fn test_ux() {
        use ux::print;

        let s = "some-repeat-text-";
        print!("{}", s.repeat(4));
        print::start_end("this is at the start", "this is at the end");
        print::end("this is at the end");
        println!("some other text");
        print::start_end(s.repeat(10), "END TEXT");
    }
    */

    #[test]
    fn test_hmerr_macro() {
        use hmerr::pfe;

        let r: Result<(), _> = pfe!("some error");
        dbg!(r.unwrap_err());
    }
}
