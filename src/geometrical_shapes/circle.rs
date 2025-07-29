use rand::Rng;
use raster::{Color, Image};
use crate::geometrical_shapes::{Drawable, Point};

#[derive(Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
    pub color: Color,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Circle {
            center,
            radius,
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

    pub fn random(_width: i32, _height: i32) -> Self {
        todo!()
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let steps: i32 = 360;
        for i in 0..360 {
            let theta = ((i as f64) * 2.0 * std::f64::consts::PI) / (steps as f64);
            let x = (self.center.x as f64) + (self.radius as f64) * theta.cos();
            let y = (self.center.y as f64) + (self.radius as f64) * theta.sin();
            let _ = image.set_pixel(x.round() as i32, y.round() as i32, self.color.clone());
        }
    }
    fn color(&self) -> &Color {
        &self.color
    }
}
