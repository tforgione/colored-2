#![allow(unused_imports)]

extern crate ansi_term;
extern crate colored;

use ansi_term::*;
use colored::*;

macro_rules! test_simple_color {
    ($string:expr, $colored_name:ident, $ansi_term_name:ident) => {{
        let s = format!("{} {}", $string, stringify!($colored_name));
        assert_eq!(s.$colored_name().to_string(), Colour::$ansi_term_name.paint(s).to_string())
    }}
}

mod compat_colors {
    use super::ansi_term::*;
    use super::colored::*;

    #[test]
    fn black() {
        test_simple_color!("test string", black, Black);
    }
    #[test]
    fn red() {
        test_simple_color!("test string", red, Red);
    }
    #[test]
    fn green() {
        test_simple_color!("test string", green, Green);
    }
    #[test]
    fn yellow() {
        test_simple_color!("test string", yellow, Yellow);
    }
    #[test]
    fn blue() {
        test_simple_color!("test string", blue, Blue);
    }
    #[test]
    fn magenta() {
        test_simple_color!("test string", magenta, Purple);
    }
    #[test]
    fn cyan() {
        test_simple_color!("test string", cyan, Cyan);
    }
    #[test]
    fn white() {
        test_simple_color!("test string", white, White);
    }
}

macro_rules! test_simple_style {
    ($string:expr, $style:ident) => {{
        let s = format!("{} {}", $string, stringify!($style));
        assert_eq!(s.$style().to_string(), ansi_term::Style::new().$style().paint(s).to_string())
    }}
}

mod compat_styles {
    use super::colored;
    use super::colored::*;
    use super::ansi_term;
    use super::ansi_term::*;

    #[test]
    fn bold() {
        test_simple_style!("test string", bold);
    }
    #[test]
    fn dimmed() {
        test_simple_style!("test string", dimmed);
    }
    #[test]
    fn italic() {
        test_simple_style!("test string", italic);
    }
    #[test]
    fn underline() {
        test_simple_style!("test string", underline);
    }
    #[test]
    fn blink() {
        test_simple_style!("test string", blink);
    }
    #[test]
    fn reverse() {
        test_simple_style!("test string", reverse);
    }
    #[test]
    fn hidden() {
        test_simple_style!("test string", hidden);
    }
}

macro_rules! test_simple_bgcolor {
    ($string:expr, $colored_name:ident, $ansi_term_name:ident) => {{
        let s = format!("{} {}", $string, stringify!($colored_name));
        assert_eq!(
            s.$colored_name().to_string(),
            ansi_term::Style::default().on(ansi_term::Colour::$ansi_term_name).paint(s).to_string()
        )
    }}
}

mod compat_bgcolors {
    use super::ansi_term;
    use super::colored;
    use super::ansi_term::*;
    use super::colored::*;

    #[test]
    fn on_black() {
        test_simple_bgcolor!("test string", on_black, Black);
    }
    #[test]
    fn on_red() {
        test_simple_bgcolor!("test string", on_red, Red);
    }
    #[test]
    fn on_green() {
        test_simple_bgcolor!("test string", on_green, Green);
    }
    #[test]
    fn on_yellow() {
        test_simple_bgcolor!("test string", on_yellow, Yellow);
    }
    #[test]
    fn on_blue() {
        test_simple_bgcolor!("test string", on_blue, Blue);
    }
    #[test]
    fn on_magenta() {
        test_simple_bgcolor!("test string", on_magenta, Purple);
    }
    #[test]
    fn on_cyan() {
        test_simple_bgcolor!("test string", on_cyan, Cyan);
    }
    #[test]
    fn on_white() {
        test_simple_bgcolor!("test string", on_white, White);
    }
}

