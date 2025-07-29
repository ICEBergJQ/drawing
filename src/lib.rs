pub mod geometrical_shapes {
    use rand::Rng;
    use raster::{ Image, Color };

    //=========== trait ========
    pub trait Drawable {
        fn draw(&self, image: &mut Image);
    }

    pub trait Displayable {
        fn display(&mut self, x: i32, y: i32, color: Color);
    }

    //=========== stucts ========
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
            Color::new(
                rng.gen_range(0..256),
                rng.gen_range(0..256),
                rng.gen_range(0..256),
                255,
            )
        }
    }
    // Point drawable
    impl Drawable for Point {
        fn draw(&self, image: &mut Image) {
            image.display(self.x, self.y, self.color);
        }
    }

    impl Line {
        pub fn new(p1: &Point, p2: &Point) -> Self {
           Line{
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
    }
    // lign drawable 
    impl Drawable for Line {
        fn draw(&self, image: &mut Image) {
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

        pub fn random(width: i32, height: i32) -> Self {

        }

    }
    impl Drawable for Circle{}
}
