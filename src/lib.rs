use rand::Rng;
use raster::{Image, Color};

pub mod geometrical_shapes {
    use raster::{Image, Color};
    use rand::Rng;

    pub trait Drawable {
        fn draw(&self, image: &mut Image);
    }

    pub trait Displayable {
        fn display(&mut self, x: i32, y: i32, color: Color);
    }

    pub struct Point {
        pub x: i32,
        pub y: i32,
        pub color: Color,
    }

    pub struct Line {
        pub start: Point,
        pub end: Point,
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
        }

        fn random_color() -> Color {
        }
    }

    impl Drawable for Point {
        fn draw(&self, image: &mut Image) {
        }
    }



    impl Line {
        pub fn new(p1: &Point, p2: &Point) -> Self {
            Self {
            }
        }

        pub fn random(width: i32, height: i32) -> Self {
        }
    }

    impl Drawable for Line {
        fn draw(&self, image: &mut Image) {
        }
    }
}
