use crate::error::DisplayError;
use embedded_graphics::drawable::Pixel;
use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::DrawTarget;
use std::convert::TryInto;
use std::marker::Unpin;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

/// Representation of the G13 LCD screen
/// It works using a AsyncWriter to flush the buffer to the real screen.
pub struct G13Display<'a, W: Unpin + AsyncWrite + 'a> {
    framebuffer: [u8; 20 * 48],
    tx: &'a mut W,
}

impl<W: Unpin + AsyncWrite> DrawTarget<BinaryColor> for G13Display<'_, W> {
    type Error = DisplayError;

    fn draw_pixel(&mut self, pixel: Pixel<BinaryColor>) -> Result<(), DisplayError> {
        let Pixel(coord, color) = pixel;
        if let Ok((x @ 0..=159u32, y @ 0..=42u32)) = coord.try_into() {
            // Offset is determined by the byte to write.
            // A byte regrouped 8 rows (in one column). The next byte is the 8 rows of the next column.
            // So: row in 0..8 start at 0, 8..16 at 160, 16..24 at 2*160, etc...
            let offset: usize = (x as usize) + ((y as usize) / 8 * 160) as usize;
            let b: u8 = color.is_on() as u8;
            // The row of pixel is determined by the position of the bit in the byte
            // (the byte is only a range of rows in a column)
            self.framebuffer[offset] |= b << (y % 8);
        }
        Ok(())
    }

    fn size(&self) -> Size {
        Size::new(160, 43)
    }
}

impl<'a, W: Unpin + AsyncWrite + 'a> G13Display<'a, W> {
    /// Create a new instance of G13Display with the given writer
    pub fn new(writer: &'a mut W) -> Self {
        G13Display {
            framebuffer: [0; 20 * 48],
            tx: writer,
        }
    }

    /// Get the inner framebuffer as a Vec<u8>
    fn to_bytes(&self) -> Vec<u8> {
        self.framebuffer.to_vec()
    }

    /// Flush the current framebuffer to the display using the inner writer
    pub async fn flush(&mut self) -> Result<(), DisplayError> {
        self.tx.write(&self.to_bytes()).await?;
        Ok(())
    }

    /// Clear the framebuffer
    pub fn clear(&mut self) {
        self.framebuffer = [0; 20 * 48];
    }
}

#[cfg(test)]
mod test {
    use crate::display::G13Display;
    use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
    use std::io::Cursor;
    use tokio::runtime::Runtime;

    /// Draw four pixels at the four corners of the screen.
    #[test]
    fn draw_corners() {
        // Create a display
        let mut writer = Cursor::new(Vec::<u8>::new());
        let mut display = G13Display::new(&mut writer);

        // Place the pixels in the corners
        let pts = [
            Point::new(0, 0),
            Point::new(159, 0),
            Point::new(0, 42),
            Point::new(159, 42),
        ];
        for pt in pts.iter() {
            Pixel(*pt, BinaryColor::On)
                .draw(&mut display)
                .expect("Should draw");
        }
        // Compare with the known result
        let mut buffer = [0; 20 * 48];
        buffer[0] = 0b00000001;
        buffer[159] = 0b00000001;
        buffer[800] = 0b00000100;
        buffer[959] = 0b00000100;
        assert_eq!(display.to_bytes(), buffer.to_vec());
    }

    /// Draw async four pixels at the four corners of the screen.
    #[test]
    fn draw_async_corners() {
        let mut rt = Runtime::new().unwrap();

        // Create a display
        let mut writer = Cursor::new(Vec::<u8>::new());
        let mut display = G13Display::new(&mut writer);

        // Place the pixels in the corners
        let pts = [
            Point::new(0, 0),
            Point::new(159, 0),
            Point::new(0, 42),
            Point::new(159, 42),
        ];
        for pt in pts.iter() {
            Pixel(*pt, BinaryColor::On)
                .draw(&mut display)
                .expect("Should draw");
        }

        // Flush display to the writer
        rt.block_on(display.flush()).unwrap();

        // Compare with the known result
        let mut buffer = [0; 20 * 48];
        buffer[0] = 0b00000001;
        buffer[159] = 0b00000001;
        buffer[800] = 0b00000100;
        buffer[959] = 0b00000100;
        assert_eq!(writer.into_inner(), buffer.to_vec());
    }
}
