use zc::{Dependant, Zc};

#[derive(Dependant)]
pub struct MyStruct<'a>(&'a [u8]);

fn main() {
    let owner = vec![1, 2, 3];
    let other = vec![1, 2, 3];
    let _ = Zc::pin(owner, |_| MyStruct(&other[..]));
}
