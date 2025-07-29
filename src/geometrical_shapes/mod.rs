pub mod circle;
pub mod line;
pub mod point;
pub mod rectangle;
pub mod triangle;
pub use circle::Circle;
pub use line::Line;
pub use point::Point;
pub use raster::{Color, Image};
pub use rectangle::Rectangle;
pub use triangle::Triangle;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> &Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub fn draw_ln(p1: Point, p2: Point, image: &mut Image, color: Color) {
    let dx = p2.x as f32 - p1.x as f32;
    let dy = p2.y as f32 - p1.y as f32;

    let steps = dx.abs().max(dy.abs()) as usize;

    let x_inc = dx / steps as f32;
    let y_inc = dy / steps as f32;

    let mut x = p1.x as f32;
    let mut y = p1.y as f32;

    for _ in 0..=steps {
        Displayable::display(image, x.round() as i32, y.round() as i32, color.clone());
        x += x_inc;
        y += y_inc;
    }
}
