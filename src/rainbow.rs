// code borrowed from https://github.com/BlueTufa/lolcat/blob/master/src/main.rs
use ansi_term::Colour;
use rand::Rng;

fn rainbow(seed: f64) -> ansi_term::Colour {
    let freq = 1.13;
    let factor = seed / 16.0;

    let red = (freq * factor as f64).sin().mul_add(127.0, 128.0) as u8;
    let green = (freq * factor + 2.0 * std::f64::consts::PI / 3.0 as f64)
        .sin()
        .mul_add(127.0, 128.0) as u8;
    let blue = (freq * factor + 4.0 * std::f64::consts::PI / 3.0 as f64)
        .sin()
        .mul_add(127.0, 128.0) as u8;

    Colour::RGB(red, green, blue)
}

pub fn print_rainbow(string: String) {
    let entropy = rand::thread_rng().gen_range(0..=255);
    for line in string.lines() {
        for (ix, c) in line.char_indices() {
            let seed = f64::from(ix as u32 + entropy);
            let colour = rainbow(seed);
            let rainbow_char = colour.paint(c.to_string());
            print!("{}", rainbow_char);
        }
        println!();
    }
}

#[macro_export]
macro_rules! print_rainbow {
    ($($arg:tt)*) => {{
        $crate::rainbow::print_rainbow(format!($($arg)*))
    }};
}
