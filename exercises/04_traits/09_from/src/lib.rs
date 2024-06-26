// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

impl<T> From<T> for WrappingU32
    where  T: PartialOrd + PartialEq{
    fn from(value: T) -> Self {
        Self::from(value)
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
