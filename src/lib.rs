use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;

#[repr(u8)]
#[derive(Clone, Copy, Debug, IntoEnumIterator, Eq, PartialEq, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1 ,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    return _color.int_value().into();
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value.try_into().unwrap()) {
        Ok(v) =>  format!("{:?}", v),
        Err(e) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
     let mut result: Vec<u8> = ResistorColor::into_enum_iter().map(|x| color_to_value(x) as u8).collect();
     result.sort();
     let new: Vec<_> = result.iter().filter_map(|x|  ResistorColor::from_int(*x).ok()).collect();
     return new;
}
