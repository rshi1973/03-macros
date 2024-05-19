use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let ret = f3(f2(f1("Hello")?)?)?;

    println!("Final Result: {:?}", ret);

    Ok(())
}

fn f1(s1: impl AsRef<str>) -> Result<String> {
    Ok(format!("f1: {}", s1.as_ref()))
}

fn f2(s2: impl AsRef<str>) -> Result<String> {
    Ok(format!("f2: {}", s2.as_ref()))
}

fn f3(s3: impl AsRef<str>) -> Result<String> {
    Err(anyhow!("f3: {}", s3.as_ref())) // Convert Error to String
}

#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        }
    };
}
