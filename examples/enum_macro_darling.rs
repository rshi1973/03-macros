use macros::EnumFromDarling;

#[derive(Debug)]
struct LeftValue(u32);

#[derive(Debug)]
struct RightValue(u32);

impl From<u32> for LeftValue {
    fn from(n: u32) -> Self {
        LeftValue(n)
    }
}

impl From<u32> for RightValue {
    fn from(n: u32) -> Self {
        RightValue(n)
    }
}

#[allow(dead_code)]
#[derive(Debug, EnumFromDarling)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(LeftValue),
    Right(RightValue),
}

#[allow(dead_code)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

fn main() {
    let up: Direction<u32> = Direction::from(DirectionUp::new(42));
    //let up: Direction<i32> = DirectionUp::new(42).into();
    let left: Direction<u32> = 10.into();

    println!("up {:?}, left {:?}", up, left);
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

impl From<u32> for Direction<u32> {
    fn from(item: u32) -> Self {
        Direction::Left(LeftValue::from(item))
    }
}
