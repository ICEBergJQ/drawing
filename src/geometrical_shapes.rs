pub mod geometrical_shapes {
    use rand::Rng;
    use raster::{ Image, Color };

    //=========== trait ========
    pub trait Drawable {
        fn draw(&self, image: &mut Image);
        fn color(&self) -> &Color;
    }

    pub trait Displayable {
        fn display(&mut self, x: i32, y: i32, color: Color);
    }

    //=========== structs ========
    #[derive(Clone)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
        pub color: Color,
    }
    
    #[derive(Clone)]
    pub struct Triangle {
        pub p1: Point,
        pub p2: Point,
        pub p3: Point,
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

    //=========== implementations =======
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
            let _ = image.set_pixel(self.x, self.y, self.color.clone());
        }
        fn color(&self) -> &Color {
            &self.color
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

    impl Triangle {
        pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
            Triangle {
                p1: p1.clone(),
                p2: p2.clone(),
                p3: p3.clone(),
                color: Self::random_color(),
            }
        }

        pub fn random(width: i32, height: i32) -> Self {
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

    // Line drawable
    impl Drawable for Line {
        fn draw(&self, _image: &mut Image) {
            let dx = (self.end.x - self.start.x).abs();
            let dy = (self.end.y - self.start.y).abs();
            let sx = if self.start.x < self.end.x { 1 } else { -1 };
            let sy = if self.start.y < self.end.y { 1 } else { -1 };
            let mut err = dx - dy;

            let mut x = self.start.x;
            let mut y = self.start.y;

            while x != self.end.x || y != self.end.y {
                let _ = _image.set_pixel(x, y, self.color.clone());
                let err2 = err * 2;
                if err2 > -dy {
                    err -= dy;
                    x += sx;
                }
                if err2 < dx {
                    err += dx;
                    y += sy;
                }
            }
        }
        fn color(&self) -> &Color {
            &self.color
        }
    }

     impl Drawable for Triangle {
        fn draw(&self, _image: &mut Image) {
            // Draw the edges of the triangle
            let line1 = Line::new(&self.p1, &self.p2);
            let line2 = Line::new(&self.p2, &self.p3);
            let line3 = Line::new(&self.p3, &self.p1);

            line1.draw(_image);
            line2.draw(_image);
            line3.draw(_image);
        }
        fn color(&self) -> &Color {
            &self.color
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
}