use embedded_graphics::fonts::{Font6x6, Font6x8};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::style::{
    PrimitiveStyle, PrimitiveStyleBuilder, TextStyle, TextStyleBuilder,
};
use ibm437::*;
use once_cell::sync::Lazy;

pub static TEXT_SMALL: Lazy<TextStyle<BinaryColor, Font6x6>> = Lazy::new(|| {
    TextStyleBuilder::new(Font6x6)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build()
});

pub static TEXT_LIGHT: Lazy<TextStyle<BinaryColor, Font6x8>> = Lazy::new(|| {
    TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build()
});

pub static TEXT_REGULAR: Lazy<TextStyle<BinaryColor, Ibm437Font8x8Normal>> = Lazy::new(|| {
    TextStyleBuilder::new(Ibm437Font8x8Normal)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build()
});

pub static TEXT_BOLD: Lazy<TextStyle<BinaryColor, Ibm437Font8x8Bold>> = Lazy::new(|| {
    TextStyleBuilder::new(Ibm437Font8x8Bold)
        .text_color(BinaryColor::On)
        .background_color(BinaryColor::Off)
        .build()
});

pub static TITLE_REGULAR: Lazy<TextStyle<BinaryColor, Ibm437Font8x8Normal>> = Lazy::new(|| {
    TextStyleBuilder::new(Ibm437Font8x8Normal)
        .text_color(BinaryColor::Off)
        .background_color(BinaryColor::On)
        .build()
});

pub static TITLE_BOLD: Lazy<TextStyle<BinaryColor, Ibm437Font8x8Bold>> = Lazy::new(|| {
    TextStyleBuilder::new(Ibm437Font8x8Bold)
        .text_color(BinaryColor::Off)
        .background_color(BinaryColor::On)
        .build()
});

pub static FILL_ON: Lazy<PrimitiveStyle<BinaryColor>> = Lazy::new(|| {
    PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::On)
        .build()
});

pub static FILL_OFF: Lazy<PrimitiveStyle<BinaryColor>> = Lazy::new(|| {
    PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::Off)
        .build()
});

pub static BORDER_1: Lazy<PrimitiveStyle<BinaryColor>> = Lazy::new(|| {
    PrimitiveStyleBuilder::new()
        .fill_color(BinaryColor::Off)
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build()
});
