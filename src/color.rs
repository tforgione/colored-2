use std::convert::From;
use std::str::FromStr;

/// A color that can be one of the 8 standard color or any RGB color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AllColor {
    Color(Color),
    True(TrueColor),
}

/// Any RGB color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrueColor(u8, u8, u8);

impl TrueColor {
    /// Creates a new RGB color from its channels values.
    pub fn new(r: u8, g: u8, b: u8) -> TrueColor {
        TrueColor(r, g, b)
    }

    /// Creates a new RGB color from its hexadecimal value.
    ///
    /// # Example
    /// ```
    /// # use colored::TrueColor;
    /// let color = TrueColor::from_hex(0xabcdef);
    /// assert_eq!(color, TrueColor::new(171, 205, 239));
    /// ```
    pub fn from_hex(hex: u64) -> TrueColor {
        let r = hex >> 16 & 0x0000ff;
        let g = hex >> 8 & 0x0000ff;
        let b = hex & 0x0000ff;
        TrueColor(r as u8, g as u8, b as u8)
    }

    pub fn to_fg_string(&self) -> String {
        format!("38;2;{};{};{}", self.0, self.1, self.2)
    }

    pub fn to_bg_string(&self) -> String {
        format!("48;2;{};{};{}", self.0, self.1, self.2)
    }
}


/// The 8 standard colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}
impl Color {
    pub fn to_fg_str(&self) -> &str {
        match *self {
            Color::Black => "30",
            Color::Red => "31",
            Color::Green => "32",
            Color::Yellow => "33",
            Color::Blue => "34",
            Color::Magenta => "35",
            Color::Cyan => "36",
            Color::White => "37",
            Color::BrightBlack => "90",
            Color::BrightRed => "91",
            Color::BrightGreen => "92",
            Color::BrightYellow => "93",
            Color::BrightBlue => "94",
            Color::BrightMagenta => "95",
            Color::BrightCyan => "96",
            Color::BrightWhite => "97",
        }
    }

    pub fn to_bg_str(&self) -> &str {
        match *self {
            Color::Black => "40",
            Color::Red => "41",
            Color::Green => "42",
            Color::Yellow => "43",
            Color::Blue => "44",
            Color::Magenta => "45",
            Color::Cyan => "46",
            Color::White => "47",
            Color::BrightBlack => "100",
            Color::BrightRed => "101",
            Color::BrightGreen => "102",
            Color::BrightYellow => "103",
            Color::BrightBlue => "104",
            Color::BrightMagenta => "105",
            Color::BrightCyan => "106",
            Color::BrightWhite => "107",
        }
    }
}

impl<'a> From<&'a str> for Color {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

impl From<String> for Color {
    fn from(src: String) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let src = src.to_lowercase();

        match src.as_ref() {
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            "magenta" => Ok(Color::Magenta),
            "cyan" => Ok(Color::Cyan),
            "white" => Ok(Color::White),
            "bright black" => Ok(Color::BrightBlack),
            "bright red" => Ok(Color::BrightRed),
            "bright green" => Ok(Color::BrightGreen),
            "bright yellow" => Ok(Color::BrightYellow),
            "bright blue" => Ok(Color::BrightBlue),
            "bright magenta" => Ok(Color::BrightMagenta),
            "bright cyan" => Ok(Color::BrightCyan),
            "bright white" => Ok(Color::BrightWhite),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    mod from_str {
        pub use super::*;

        macro_rules! make_test {
            ( $( $name:ident: $src:expr => $dst:expr),* ) => {

                $(
                    #[test]
                    fn $name() {
                        let color : Color = $src.into();
                        assert_eq!($dst, color)
                    }
                )*
            }
        }

        make_test!(
            black: "black" => Color::Black,
            red: "red" => Color::Red,
            green: "green" => Color::Green,
            yellow: "yellow" => Color::Yellow,
            blue: "blue" => Color::Blue,
            magenta: "magenta" => Color::Magenta,
            cyan: "cyan" => Color::Cyan,
            white: "white" => Color::White,
            brightblack: "bright black" => Color::BrightBlack,
            brightred: "bright red" => Color::BrightRed,
            brightgreen: "bright green" => Color::BrightGreen,
            brightyellow: "bright yellow" => Color::BrightYellow,
            brightblue: "bright blue" => Color::BrightBlue,
            brightmagenta: "bright magenta" => Color::BrightMagenta,
            brightcyan: "bright cyan" => Color::BrightCyan,
            brightwhite: "bright white" => Color::BrightWhite,

            invalid: "invalid" => Color::White,
            capitalized: "BLUE" => Color::Blue,
            mixed_case: "bLuE" => Color::Blue
        );
    }

    mod from_string {
        pub use super::*;

        macro_rules! make_test {
            ( $( $name:ident: $src:expr => $dst:expr),* ) => {

                $(
                    #[test]
                    fn $name() {
                        let src = String::from($src);
                        let color : Color = src.into();
                        assert_eq!($dst, color)
                    }
                )*
            }
        }

        make_test!(
            black: "black" => Color::Black,
            red: "red" => Color::Red,
            green: "green" => Color::Green,
            yellow: "yellow" => Color::Yellow,
            blue: "blue" => Color::Blue,
            magenta: "magenta" => Color::Magenta,
            cyan: "cyan" => Color::Cyan,
            white: "white" => Color::White,
            brightblack: "bright black" => Color::BrightBlack,
            brightred: "bright red" => Color::BrightRed,
            brightgreen: "bright green" => Color::BrightGreen,
            brightyellow: "bright yellow" => Color::BrightYellow,
            brightblue: "bright blue" => Color::BrightBlue,
            brightmagenta: "bright magenta" => Color::BrightMagenta,
            brightcyan: "bright cyan" => Color::BrightCyan,
            brightwhite: "bright white" => Color::BrightWhite,

            invalid: "invalid" => Color::White,
            capitalized: "BLUE" => Color::Blue,
            mixed_case: "bLuE" => Color::Blue
        );
    }

    mod fromstr {
        pub use super::*;

        #[test]
        fn parse() {
            let color: Result<Color, _> = "blue".parse();
            assert_eq!(Ok(Color::Blue), color)
        }

        #[test]
        fn error() {
            let color: Result<Color, ()> = "bloublou".parse();
            assert_eq!(Err(()), color)
        }

    }
}
