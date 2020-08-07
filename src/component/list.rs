use crate::error::DisplayError;
use crate::style::TEXT_REGULAR;
use embedded_graphics::drawable::Drawable;
use embedded_graphics::drawable::Pixel;
use embedded_graphics::fonts::Text;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::DrawTarget;

/// A list component for g13 apps
#[derive(Clone, Debug)]
pub struct List {
    cursor: usize,
    content: Vec<String>,
}

impl List {
    /// Init a new list component with a vec of choices
    pub fn new(content: Vec<String>) -> Self {
        Self { cursor: 0, content }
    }

    /// Get the current cursor
    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    /// Get the current choice selected
    pub fn get_current(&self) -> &str {
        self.content.get(self.cursor).unwrap()
    }

    /// Change the cursor for the next choice
    pub fn next(&mut self) {
        if self.cursor < self.content.len() - 1 {
            self.cursor += 1;
        }
    }

    /// Change the cursor for the previous choice
    pub fn previous(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Draw a list with its default size annd position
    /// Currently, it's (0,10) to (159,34)
    pub fn draw_default<D>(&self, display: &mut D) -> Result<(), DisplayError>
    where
        D: DrawTarget<BinaryColor, Error = DisplayError>,
    {
        self.draw_within_border(display, Point::new(0, 10), Point::new(159, 34))
    }

    /// Draw a list in the given border, defined by the points given of a rectangle.
    pub fn draw_within_border<D: DrawTarget<BinaryColor>>(
        &self,
        display: &mut D,
        up_corner: Point,
        down_corner: Point,
    ) -> Result<(), D::Error> {
        let mut pixels: Vec<Pixel<BinaryColor>> = Vec::new();
        let (left_offset, up_offset) = (up_corner.x, up_corner.y);
        let (right_offset, down_offset) = (down_corner.x, down_corner.y);
        let nshow: i32 = (down_offset - up_offset) / 8;
        // For each visible choice
        for i in 0..nshow {
            // If cursor is 0, element -1 does not exist - just pass
            if self.cursor == 0 && i == 0 {
                continue;
            }
            // Print the choice name
            if let Some(name) = self.content.get(self.cursor + (i as usize) - 1) {
                let prefix = if i == 1 { ">" } else { " " }; // add the prefix if the choice is selected.
                pixels.extend(
                    Text::new(
                        &format!("{} {}", prefix, name),
                        Point::new(left_offset + 1, up_offset + 8 * i),
                    )
                    .into_styled(*TEXT_REGULAR)
                    .into_iter()
                    // Remove the overflow
                    .filter(|pixel| {
                        left_offset <= pixel.0.x
                            && pixel.0.x <= right_offset
                            && up_offset <= pixel.0.y
                            && pixel.0.y <= down_offset
                    }),
                );
            }
        }
        // draw all visible choices
        pixels.into_iter().draw(display)
    }
}
