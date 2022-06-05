use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(usize)]
#[derive(Debug, PartialEq, Clone, Copy, Eq, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn _value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value) {
        Ok(x) => match x {
            ResistorColor::Black => "Black",
            ResistorColor::Blue => "Blue",
            ResistorColor::Brown => "Brown",
            ResistorColor::Green => "Green",
            ResistorColor::Grey => "Gray",
            ResistorColor::Orange => "Orange",
            ResistorColor::Red => "Red",
            ResistorColor::Violet => "Violet",
            ResistorColor::White => "White",
            ResistorColor::Yellow => "Yellow",
        }.to_string(),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn _colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
