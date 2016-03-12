
use std::ops::Deref;

const CLEARV    : u8 = 0b0000_0000;
const BOLD      : u8 = 0b0000_0001;
const UNDERLINE : u8 = 0b0000_0010;
const REVERSED  : u8 = 0b0000_0100;
const ITALIC    : u8 = 0b0000_1000;
const BLINK     : u8 = 0b0001_0000;
const HIDDEN    : u8 = 0b0010_0000;

#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub struct Style(u8);

impl Style {
    pub fn to_str(&self) -> &str {
        unreachable!()
    }
}

pub static CLEAR : Style = Style(CLEARV);
