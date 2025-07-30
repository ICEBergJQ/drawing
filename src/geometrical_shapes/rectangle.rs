use crate::geometrical_shapes::{draw_ln, Drawable, Point};
use rand::Rng;
use raster::{Color, Image};

pub struct Rectangle {
    pub tl: Point,
    pub br: Point,
    pub color: Color,
}

impl Rectangle {
    pub fn new(top_left: &Point, bottom_right: &Point) -> Self {
        Self {
            tl: top_left.clone(),
            br: bottom_right.clone(),
            color: Self::random_color(),
        }
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

    pub fn _random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let x1 = rng.gen_range(0..width);
        let y1 = rng.gen_range(0..height);
        let x2 = rng.gen_range(x1..width);
        let y2 = rng.gen_range(y1..height);
        Rectangle::new(&Point::new(x1, y1), &Point::new(x2, y2))
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let tr = Point::new(self.br.x, self.tl.y);
        let bl = Point::new(self.tl.x, self.br.y);

        draw_ln(self.tl.clone(), tr.clone(), image, self.color().clone());
        draw_ln(tr, self.br.clone(), image, self.color().clone());
        draw_ln(self.br.clone(), bl.clone(), image, self.color().clone());
        draw_ln(bl, self.tl.clone(), image, self.color().clone());
    }
    fn color(&self) -> &Color {
        &self.color
    }
}
