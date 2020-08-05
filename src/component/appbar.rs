use crate::style::{FILL_ON, TITLE_BOLD};
use embedded_graphics::drawable::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::DrawTarget;
use embedded_graphics::{egrectangle, egtext};

/// An app bar component
/// It's essentially a black bold title on a white background
#[derive(Clone, Debug)]
pub struct AppBar {
    top_left: Point,
    bottom_right: Point,
    title: String,
}

impl AppBar {
    /// Create an app bar component from the title text and its position
    pub fn new<T: Into<String>>(title: T, top_left: Point, bottom_right: Point) -> Self {
        Self {
            top_left,
            bottom_right,
            title: title.into(),
        }
    }
}

impl Drawable<BinaryColor> for AppBar {
    fn draw<D: DrawTarget<BinaryColor>>(self, display: &mut D) -> Result<(), D::Error> {
        self.into_iter().draw(display)?;
        Ok(())
    }
}

impl IntoIterator for AppBar {
    type Item = Pixel<BinaryColor>;
    type IntoIter = std::vec::IntoIter<Pixel<BinaryColor>>;

    fn into_iter(self) -> Self::IntoIter {
        // Draw the white background
        egrectangle!(
            top_left = self.top_left,
            bottom_right = self.bottom_right,
            style = *FILL_ON,
        )
        .into_iter()
        .chain(
            // and chain with the black bold title (with an offset of 1;1)
            egtext!(
                text = &self.title,
                top_left = Point::from((1, 1)) + self.top_left,
                style = *TITLE_BOLD,
            )
            .into_iter()
            // Filter to get only the pixel in the appbar container
            .filter(|pixel| {
                self.top_left.x <= pixel.0.x
                    && pixel.0.x <= self.bottom_right.x
                    && self.top_left.y <= pixel.0.y
                    && pixel.0.y <= self.bottom_right.y
            }),
        )
        .collect::<Vec<_>>()
        .into_iter()
    }
}
