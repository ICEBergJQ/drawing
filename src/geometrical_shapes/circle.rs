use rand::Rng;
use raster::{Color, Image};
use crate::geometrical_shapes::{Drawable, Point,Displayable};

#[derive(Clone)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
    pub color: Color,
}

impl Circle {
    #[allow(dead_code)]
    pub fn new(center: Point, radius: i32) -> Self {
        Circle {
            center,
            radius,
            color: Self::random_color(),
        }
    }
    pub fn random_color() -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen_range(0..=255),
            g: rng.gen_range(0..=255),
            b: rng.gen_range(0..=255),
            a: 255,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let center = Point::new(rng.gen_range(0..width), rng.gen_range(0..height));
        let radius = rng.gen_range(10..200);
        Circle::new(center, radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let steps: i32 = 180*  self.radius;
        let color =  self.color();
        for i in 0..steps {
            let theta: f64 = ((i as f64) * std::f64::consts::PI) / (steps as f64);
            let x = (self.center.x as f64) + (self.radius as f64) * theta.cos();
            let y = (self.center.y as f64) + (self.radius as f64) * theta.sin();
            let minus_y  = (self.center.y as f64) - (self.radius as f64) * theta.sin();
            Displayable::display(image, x.round()  as i32, y.round() as i32, color.clone());
            Displayable::display(image, x.round() as i32, minus_y.round() as i32, color.clone());

        }
    }
    fn color(&self) -> &Color {
        &self.color
    }
}
