use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::style::{
    PrimitiveStyle, PrimitiveStyleBuilder, TextStyle, TextStyleBuilder,
};
use ibm437::*;
use once_cell::sync::Lazy;

pub static TEXT_REGULAR: Lazy<TextStyle<BinaryColor, Ibm437Font8x8Normal>> = Lazy::new(|| {
    TextStyleBuilder::new(Ibm437Font8x8Normal)
        .text_color(BinaryColor::On)
        .build()
});

pub static TEXT_BOLD: Lazy<TextStyle<BinaryColor, Ibm437Font8x8Bold>> = Lazy::new(|| {
    TextStyleBuilder::new(Ibm437Font8x8Bold)
        .text_color(BinaryColor::On)
        .build()
});

pub static TITLE_REGULAR: Lazy<TextStyle<BinaryColor, Ibm437Font8x8Normal>> = Lazy::new(|| {
    TextStyleBuilder::new(Ibm437Font8x8Normal)
        .text_color(BinaryColor::Off)
        .build()
});

pub static TITLE_BOLD: Lazy<TextStyle<BinaryColor, Ibm437Font8x8Bold>> = Lazy::new(|| {
    TextStyleBuilder::new(Ibm437Font8x8Bold)
        .text_color(BinaryColor::Off)
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
