use anyhow::Result;

fn main() -> Result<()> {
    let v: Vec<i32> = my_vec!["1".parse()?, "2".parse()?, "3".parse()?,];

    println!("{:?}", v);

    Ok(())
}

// my_vec! = my_vec! { 1, 2, 3, 4, 5 }; // Vec<i32>
#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    ($($x:expr), + $(,)?) => {
        {
            <[_]>::into_vec(Box::new([$($x), +]))
        }
    };
}
