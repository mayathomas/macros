use macros::EnumFrom;

#[derive(Debug, EnumFrom)]
#[allow(unused)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
}

#[derive(Debug)]
#[allow(unused)]
struct DirectionUp {
    speed: u32,
}

impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}

fn main() {
    let up: Direction = DirectionUp::new(42).into();
    let left: Direction = 20.into();
    println!("{:?},{:?}", up, left);
}
