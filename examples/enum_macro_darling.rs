use macros::EnumFromDarling;

#[derive(Debug, EnumFromDarling)]
#[allow(unused)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
}

#[derive(Debug)]
#[allow(unused)]
struct DirectionUp<T> {
    speed: T,
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

fn main() {
    let up: Direction<u32> = DirectionUp::new(42).into();
    let left: Direction<u32> = 20.into();
    println!("{:?},{:?}", up, left);
}
