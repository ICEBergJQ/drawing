use rand::Rng;
use raster::{Color, Image};
use crate::geometrical_shapes::{Drawable, Point, draw_ln};


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

    pub fn _random(_width: i32, _height: i32) -> Self {
        todo!()
    }
}

impl Drawable for Rectangle {
    fn draw(&self, _image: &mut Image) {
        let tr = Point::new(self.br.x, self.tl.y);
        let bl = Point::new(self.tl.x, self.br.y);

        draw_ln(self.tl.clone(), tr.clone(), _image, self.color.clone());
        draw_ln(tr, self.br.clone(), _image, self.color.clone());
        draw_ln(self.br.clone(), bl.clone(), _image, self.color.clone());
        draw_ln(bl, self.tl.clone(), _image, self.color.clone());
    }
    fn color(&self) -> &Color {
        &self.color
    }
}
