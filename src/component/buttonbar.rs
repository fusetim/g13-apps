use crate::style::TEXT_BOLD;
use embedded_graphics::drawable::Drawable;
use embedded_graphics::drawable::Pixel;
use embedded_graphics::fonts::Text;
use embedded_graphics::geometry::Dimensions;
use embedded_graphics::geometry::Point;
use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::transform::Transform;
use embedded_graphics::DrawTarget;

/// The constant height of a button, here 8px.
const BUTTON_HEIGHT: u32 = 8;
/// The constant width of a button, here 40px.
const BUTTON_WIDTH: u32 = 40; // 160 / 4
/// The height where the button bar will be drawed.
/// It's 35px, to draw on the G13 last line.
const HEIGHT_OFFSET: i32 = 35;

/// A button component.
/// Mainly used with the ButtonBar component
#[derive(Clone, Debug)]
pub struct Button {
    top_left: Point,
    bottom_right: Point,
    pixels: Vec<Pixel<BinaryColor>>,
}

impl Button {
    /// Create a button from an existing drawable.
    /// &Drawable does not implement Dimensions, so you have to set them
    pub fn from_drawable<D: IntoIterator<Item = Pixel<BinaryColor>>>(
        drawable: D,
        top_left: Point,
        bottom_right: Point,
    ) -> Button {
        Button {
            top_left,
            bottom_right,
            pixels: drawable.into_iter().collect(),
        }
    }

    /// Create a text button with the given str
    pub fn from_str(text: &str) -> Button {
        let comp = Text::new(text, Point::zero()).into_styled(*TEXT_BOLD);
        Button::from_drawable(&comp, comp.top_left(), comp.bottom_right())
    }
}

impl Drawable<BinaryColor> for Button {
    fn draw<D: DrawTarget<BinaryColor>>(self, display: &mut D) -> Result<(), D::Error> {
        self.pixels.into_iter().draw(display)
    }
}

impl Transform for Button {
    fn translate(&self, by: Point) -> Self {
        Self {
            top_left: self.top_left + by,
            bottom_right: self.bottom_right + by,
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
        Size::new(
            (self.bottom_right.x - self.top_left.x).abs() as u32,
            (self.bottom_right.y - self.top_left.y).abs() as u32,
        )
    }
}

impl IntoIterator for Button {
    type Item = Pixel<BinaryColor>;
    type IntoIter = std::vec::IntoIter<Pixel<BinaryColor>>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

/// A Button bar component.
/// Can handle up to 4 buttons.
#[derive(Default, Clone, Debug)]
pub struct ButtonBar {
    button1: Option<Button>,
    button2: Option<Button>,
    button3: Option<Button>,
    button4: Option<Button>,
}

impl ButtonBar {
    /// Create a buttonbar with the givenn button.
    /// A button set to None, will not be drawed
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

    /// Set the button 1 in the button bar
    pub fn set_button1(&mut self, btn1: Option<Button>) {
        self.button1 = btn1;
    }

    /// Set the button 2 in the button bar
    pub fn set_button2(&mut self, btn2: Option<Button>) {
        self.button2 = btn2;
    }

    /// Set the button 3 in the button bar
    pub fn set_button3(&mut self, btn3: Option<Button>) {
        self.button3 = btn3;
    }

    /// Set the button 4 in the button bar
    pub fn set_button4(&mut self, btn4: Option<Button>) {
        self.button4 = btn4;
    }
}

impl Drawable<BinaryColor> for ButtonBar {
    fn draw<T: DrawTarget<BinaryColor>>(self, display: &mut T) -> Result<(), T::Error> {
        self.into_iter().draw(display)
    }
}

impl IntoIterator for ButtonBar {
    type Item = Pixel<BinaryColor>;
    type IntoIter = std::vec::IntoIter<Pixel<BinaryColor>>;

    fn into_iter(self) -> Self::IntoIter {
        /// Will tranform the button and return its pixels
        fn button_pixel_iter(btn: Option<Button>, offset: Point) -> Vec<Pixel<BinaryColor>> {
            match btn {
                Some(btn) => {
                    // Get the free space
                    let free = Size::new(BUTTON_WIDTH, BUTTON_HEIGHT) - btn.size();
                    // Center the button in its place
                    let pos: Point = offset + (free / 2);
                    btn.translate(pos).into_iter().collect() // and collect its pixels
                }
                None => Vec::with_capacity(0),
            }
        }
        // Chain all the buttons pixel
        button_pixel_iter(
            self.button1,
            Point::new(0 * BUTTON_WIDTH as i32, HEIGHT_OFFSET),
        )
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
