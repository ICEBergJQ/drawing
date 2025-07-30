use rand::Rng;
use raster::{Color, Image};
use crate::geometrical_shapes::{Drawable, Point};


pub struct Rectangle {
    pub _top_left: Point,
    pub _width: i32,
    pub _height: i32,
    pub color: Color,
}

impl Rectangle {
    pub fn _new(_center: Point, _radius: i32) -> Self {
        todo!()
    }
    fn _random_color() -> Color {
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
        todo!()
    }
    fn color(&self) -> &Color {
        &self.color
    }
}
