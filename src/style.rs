
const CLEARV    : u8 = 0b0000_0000;
const BOLD      : u8 = 0b0000_0001;
const UNDERLINE : u8 = 0b0000_0010;
const REVERSED  : u8 = 0b0000_0100;
const ITALIC    : u8 = 0b0000_1000;
const BLINK     : u8 = 0b0001_0000;
const HIDDEN    : u8 = 0b0010_0000;
const DIMMED    : u8 = 0b0100_0000;

static STYLES : [(u8, Styles); 7] = [
    (BOLD, Styles::Bold),
    (DIMMED, Styles::Dimmed),
    (UNDERLINE, Styles::Underline),
    (REVERSED, Styles::Reversed),
    (ITALIC, Styles::Italic),
    (BLINK, Styles::Blink),
    (HIDDEN, Styles::Hidden),
];

pub static CLEAR : Style = Style(CLEARV);


#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub struct Style(u8);

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum Styles {
    Clear,
    Bold,
    Dimmed,
    Underline,
    Reversed,
    Italic,
    Blink,
    Hidden
}

impl Styles {
    fn to_str<'a>(self) -> &'a str {
        match self {
            Styles::Clear => "", // unreachable, but we don't want to panic
            Styles::Bold => "1",
            Styles::Dimmed => "2",
            Styles::Italic => "3",
            Styles::Underline => "4",
            Styles::Blink => "5",
            Styles::Reversed => "6",
            Styles::Hidden => "7",
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            Styles::Clear => CLEARV,
            Styles::Bold => BOLD,
            Styles::Dimmed => DIMMED,
            Styles::Italic => ITALIC,
            Styles::Underline => UNDERLINE,
            Styles::Blink => BLINK,
            Styles::Reversed => REVERSED,
            Styles::Hidden => HIDDEN,
        }
    }

    fn from_u8(u: u8) -> Option<Vec<Styles>> {
        if u == CLEARV {
            return None;
        }

        let res : Vec<Styles> = STYLES.into_iter()
            .filter(|&&(ref mask,_)| (0 != (u & mask)))
            .map(|&(_,value)| value)
            .collect();
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }
}

impl Style {
    pub fn to_str(&self) -> String {

        let styles = match Styles::from_u8(self.0) {
            None => return String::new(),
            Some(s) => s
        };
        let mut res = String::new();
        let mut first = true;

        for style in styles.iter().map(|s| s.to_str()) {
            if first {
                res.push_str(style);
                first = false;
                continue;
            } else {
                res.push(';');
                res.push_str(style)
            }
        }
        res
    }

    pub fn new(from: Styles) -> Style {
        Style(from.to_u8())
    }

    pub fn from_both(one: Style, two: Styles) -> Style {
        Style(one.0 | two.to_u8())
    }
}
