use raster::{Color, Image};
use rand::Rng;
use crate::geometrical_shapes::{Drawable, Displayable};
  
  #[derive(Clone)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
        pub color: Color,
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Self {
            Point {
                x,
                y,
                color: Self::random_color(),
            }
        }

        pub fn random(width: i32, height: i32) -> Self {
            let mut rng = rand::thread_rng();
            Point::new(rng.gen_range(0..width), rng.gen_range(0..height))
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

    // Point drawable
    impl Drawable for Point {
        fn draw(&self, image: &mut Image) {
            // let _ = image.set_pixel(self.x, self.y, self.color.clone());
            Displayable::display(image, self.x, self.y, self.color.clone());
        }
        fn color(&self) -> &Color {
            &self.color
        }
    }