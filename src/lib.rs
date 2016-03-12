#![feature(plugin)]
#![plugin(clippy)]
#![warn(clippy_pedantic)]
#![allow(print_stdout)]

#[cfg(test)]
#[macro_use(expect)]
extern crate expectest;

use std::convert::From;
use std::ops::Deref;
use std::string::String;
use std::fmt;

#[derive(Debug,PartialEq, Eq)]
enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}

#[derive(Debug,PartialEq, Eq)]
enum Styles {
    Clear       = 0b0000_0000,
    Bold        = 0b0000_0001,
    Underline   = 0b0000_0010,
    Reversed    = 0b0000_0100,
    Italic      = 0b0000_1000,
    Blink       = 0b0001_0000,
    Hidden      = 0b0010_0000
}

#[derive(Debug,PartialEq,Eq)]
struct Style(u8);

/// Colored mean both color or styled
#[derive(Debug,PartialEq, Eq)]
pub struct ColoredString {
    input: String,
    fgcolor: Option<Color>,
    bgcolor: Option<Color>,
    style: Style
}

pub trait Colorize {
    // Colors
    //fn black(self) -> ColoredString;
    fn red(self) -> ColoredString;
    /*
    fn green(self) -> ColoredString;
    fn yellow(self) -> ColoredString;
    fn blue(self) -> ColoredString;
    fn magenta(self) -> ColoredString;
    fn purple(self) -> ColoredString;
    fn cyan(self) -> ColoredString;
    fn white(self) -> ColoredString;
    // Styles
    fn clear(self) -> ColoredString;
    fn normal(self) -> ColoredString;
    fn bold(self) -> ColoredString;
    fn dimmed(self) -> ColoredString;
    fn italic(self) -> ColoredString;
    fn underline(self) -> ColoredString;
    fn blink(self) -> ColoredString;
    fn reverse(self) -> ColoredString;
    fn hidden(self) -> ColoredString;
    */
}

impl ColoredString {
    pub fn is_plain(&self) -> bool {
        (self.bgcolor.is_none() && self.fgcolor.is_none()
            && self.style == Style(Styles::Clear as u8))
    }
}

impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: None,
            bgcolor: None,
            style: Style(0)
        }
    }
}

impl Deref for ColoredString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.input
    }
}

impl<'a> From<&'a str> for ColoredString {
    fn from(s: &'a str) -> Self {
        ColoredString {
            input: String::from(s), .. ColoredString::default()
        }
    }
}

impl Colorize for ColoredString {
    fn red(self) -> ColoredString {
        ColoredString {
            fgcolor: Some(Color::Red), .. self
        }
    }
}

impl<'a> Colorize for &'a str {
    fn red(self) -> ColoredString {
        ColoredString {
            input: String::from(self),
            fgcolor: Some(Color::Red),
            .. ColoredString::default()
        }
    }
}

impl Color {
    fn to_fg_str(&self) -> &str {
        use Color::*;
        match *self {
            Black => "30",
            Red => "31",
            Green => "32",
            Yellow => "33",
            Blue => "34",
            Magenta => "35",
            Cyan => "36",
            White => "37"
        }
    }

    fn to_bg_str(&self) -> &str {
        use Color::*;
        match *self {
            Black => "40",
            Red => "41",
            Green => "42",
            Yellow => "43",
            Blue => "44",
            Magenta => "45",
            Cyan => "46",
            White => "47"
        }
    }
}

impl Style {
    fn to_str(&self) -> &str {
        unreachable!()
    }
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_plain() {
            try!(f.write_str(&self.input));
            return Ok(())
        }

        try!(f.write_str("\x1B["));

        if self.style != Style(0) {
            try!(f.write_str(self.style.to_str()));
            try!(f.write_str(";"))
        }

        if let Some(ref color) = self.bgcolor {
            try!(f.write_str(color.to_bg_str()));
            try!(f.write_str(";"))
        }

        if let Some(ref color) = self.fgcolor {
            try!(f.write_str(color.to_fg_str()))
        }

        try!(f.write_str("m"));
        try!(f.write_str(&self.input));
        try!(f.write_str("\x1B[0m"));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expectest::prelude::*;

    #[test]
    fn it_works() {
        let toto = "toto";
        println!("{}", toto.red());
        println!("{}", String::from(toto).red());
        /*
        println!("{}", toto.blue());
        println!("{}", toto.blue().bold());
        println!("{}", toto.green());
        println!("{}", toto.yellow());
        println!("{}", toto.purple());
        println!("{}", toto.magenta());
        println!("{}", toto.cyan());
        println!("{}", toto.white());
        println!("{}", toto.white().red().blue().green());
        */
        assert!(false)
        //expect!(Style::default().paint("default").red().to_string()).to(be_equal_to("default"));
        // assert!("plop".red().to_string() != "plop")
    }
}
