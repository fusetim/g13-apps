use embedded_graphics::drawable::Drawable;
use embedded_graphics::drawable::Pixel;
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::geometry::Point;
use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::transform::Transform;
use embedded_graphics::DrawTarget;
use embedded_graphics::fonts::Text;
use crate::style::TEXT_BOLD;

const BUTTON_HEIGHT: u32 = 8;
const BUTTON_WIDTH: u32 = 40; // 160 / 4
const HEIGHT_OFFSET : i32 = 35;

#[derive(Clone, Debug)]
pub struct Button {
    top_left: Point,
    bottom_right: Point,
    size: Size,
    pixels: Vec<Pixel<BinaryColor>>,
}

impl Button {
    pub fn from_drawable<
        D: IntoIterator<Item = Pixel<BinaryColor>>,
    >(
        drawable: D, top_left: Point, bottom_right: Point
    ) -> Button {
        Button {
            top_left,
            bottom_right,
            size: Size::new((bottom_right.x - top_left.x).abs() as u32, (bottom_right.y - top_left.y).abs() as u32),
            pixels: drawable.into_iter().collect(),
        }
    }

    pub fn from_str(text: &str) -> Button {
        let comp = Text::new(text, Point::zero()).into_styled(*TEXT_BOLD);
        Button::from_drawable(&comp, comp.top_left(), comp.bottom_right())
    }
}

impl Drawable<BinaryColor> for Button {
    fn draw<D: DrawTarget<BinaryColor>>(self, display: &mut D) -> Result<(), D::Error> {
        self.pixels.into_iter().draw(display)?;
        Ok(())
    }
}

impl Transform for Button {
    fn translate(&self, by: Point) -> Self {
        Self {
            top_left: self.top_left + by,
            bottom_right: self.bottom_right + by,
            size: self.size,
            pixels: self
                .pixels
                .iter()
                .map(|pixel| Pixel(pixel.0 + by, pixel.1))
                .collect(),
        }
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.top_left += by;
        self.bottom_right += by;
        self.pixels = self
            .pixels
            .iter()
            .map(|pixel| Pixel(pixel.0 + by, pixel.1))
            .collect();
        self
    }
}

impl Dimensions for Button {
    fn top_left(&self) -> Point {
        self.top_left
    }

    fn bottom_right(&self) -> Point {
        self.bottom_right
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl IntoIterator for Button {
    type Item = Pixel<BinaryColor>;
    type IntoIter = std::vec::IntoIter<Pixel<BinaryColor>>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

#[derive(Default, Clone, Debug)]
pub struct ButtonBar {
    button1: Option<Button>,
    button2: Option<Button>,
    button3: Option<Button>,
    button4: Option<Button>,
}

impl ButtonBar {
    pub fn new(
        btn1: Option<Button>,
        btn2: Option<Button>,
        btn3: Option<Button>,
        btn4: Option<Button>,
    ) -> Self {
        Self {
            button1: btn1,
            button2: btn2,
            button3: btn3,
            button4: btn4,
        }
    }

    pub fn set_button1(&mut self, btn1: Option<Button>) {
        self.button1 = btn1;
    }
    pub fn set_button2(&mut self, btn2: Option<Button>) {
        self.button2 = btn2;
    }
    pub fn set_button3(&mut self, btn3: Option<Button>) {
        self.button3 = btn3;
    }
    pub fn set_button4(&mut self, btn4: Option<Button>) {
        self.button4 = btn4;
    }
}

impl Drawable<BinaryColor> for ButtonBar {
    fn draw<T: DrawTarget<BinaryColor>>(self, display: &mut T) -> Result<(), T::Error> {
        self.into_iter().draw(display)?;
        Ok(())
    }
}

impl IntoIterator for ButtonBar {
    type Item = Pixel<BinaryColor>;
    type IntoIter = std::vec::IntoIter<Pixel<BinaryColor>>;

    fn into_iter(self) -> Self::IntoIter {
        fn button_pixel_iter(btn: Option<Button>, offset: Point) -> Vec<Pixel<BinaryColor>> {
            match btn {
                Some(btn) => {
                    let free = Size::new(BUTTON_WIDTH, BUTTON_HEIGHT) - btn.size();
                    let pos: Point = offset + (free / 2);
                    btn.translate(pos).into_iter().collect()
                }
                None => Vec::with_capacity(0),
            }
        }
        button_pixel_iter(self.button1, Point::new(0 * BUTTON_WIDTH as i32, HEIGHT_OFFSET))
            .into_iter()
            .chain(button_pixel_iter(
                self.button2,
                Point::new(1 * BUTTON_WIDTH as i32, HEIGHT_OFFSET),
            ))
            .chain(button_pixel_iter(
                self.button3,
                Point::new(2 * BUTTON_WIDTH as i32, HEIGHT_OFFSET),
            ))
            .chain(button_pixel_iter(
                self.button4,
                Point::new(3 * BUTTON_WIDTH as i32, HEIGHT_OFFSET),
            ))
            .collect::<Vec<_>>()
            .into_iter()
    }
}
