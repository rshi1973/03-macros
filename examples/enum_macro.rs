use macros::EnumFrom;

#[allow(dead_code)]
#[derive(EnumFrom, Debug)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right(u32, u32),
}

#[allow(dead_code)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

fn main() {
    let up = Direction::from(DirectionUp::new(42));
    let left: Direction<u32> = 10.into();

    println!("up {:?}, left {:?}", up, left);
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}
