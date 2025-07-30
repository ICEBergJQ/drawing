use rand::Rng;
use raster::{Color, Image};
use crate::geometrical_shapes::{Drawable, draw_ln, Point};

pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    pub color: Color,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle {
            p1: p1.clone(),
            p2: p2.clone(),
            p3: p3.clone(),
            color: Self::random_color(),
        }
    }

    pub fn _random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let p1 = Point::new(rng.gen_range(0..width), rng.gen_range(0..height));
        let p2 = Point::new(rng.gen_range(0..width), rng.gen_range(0..height));
        let p3 = Point::new(rng.gen_range(0..width), rng.gen_range(0..height));
        Triangle::new(&p1, &p2, &p3)
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

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        draw_ln(self.p1.clone(), self.p2.clone(), image, self.color().clone());
        draw_ln(self.p2.clone(), self.p3.clone(), image, self.color().clone());
        draw_ln(self.p3.clone(), self.p1.clone(), image, self.color().clone());
    }
    fn color(&self) -> &Color {
        &self.color
    }
}
