pub mod geometrical_shapes {
    use rand::Rng;
    use raster::{ Image, Color };

    //=========== trait ========
    pub trait Drawable {
        fn draw(&self, image: &mut Image);
        fn color(&self) -> Color;
    }

    pub trait Displayable {
        fn display(&mut self, x: i32, y: i32, color: Color);
    }

    //=========== stucts ========
    #[derive(Clone)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
        pub color: Color,
    }

    #[derive(Clone)]
    pub struct Line {
        pub start: Point,
        pub end: Point,
        pub color: Color,
    }

    #[derive(Clone)]
    pub struct Circle {
        pub center: Point,
        pub radius: i32,
        pub color: Color,
    }

    //=========== implimentations =======
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
            let _ = image.set_pixel(self.x, self.y, self.color);
        }
        fn color(&self) -> Color {
            self.color
        }
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
    // lign drawable
    impl Drawable for Line {
        fn draw(&self, image: &mut Image) {}
        fn color(&self) -> Color {
            self.color
        }
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

        pub fn random(width: i32, height: i32) -> Self {
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
                let _ = image.set_pixel(x.round() as i32, y.round() as i32, self.color);
            }
        }
        fn color(&self) -> Color {
            self.color
        }
    }
}
