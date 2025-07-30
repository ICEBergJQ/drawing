use crate::geometrical_shapes::point::Point;
use crate::geometrical_shapes::{Drawable, draw_ln};
use rand::Rng;
use raster::{Color, Image};
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: Color,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Line {
            start: p1.clone(),
            end: p2.clone(),
            color: Self::random_color(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let start = Point::new(rng.gen_range(0..width), rng.gen_range(0..height));
        let end = Point::new(rng.gen_range(0..width), rng.gen_range(0..height));
        Line::new(&start, &end)
    }
    fn random_color() -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
            a: 255,
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        draw_ln(
            self.start.clone(),
            self.end.clone(),
            image,
            self.color.clone(),
        );
    }
    fn color(&self) -> &Color {
        &self.color
    }
}
