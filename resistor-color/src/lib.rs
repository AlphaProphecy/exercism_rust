use enum_iterator::{all, Sequence};
use int_enum::IntEnum;
use std::fmt;

#[repr(i8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value() as u32
}

pub fn value_to_color_string(value: u32) -> String {
    match ResistorColor::from_int(value as i8) {
        Ok(v) => v.to_string(),
        Err(_) => "value out of range".to_owned(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut v = all::<ResistorColor>().collect::<Vec<_>>();
    v.sort_by(|a, b| a.int_value().partial_cmp(&b.int_value()).unwrap());
    v
}
