//! Work-in-progress showcase for how GATs will change the world.

#![feature(generic_associated_types)]

use std::slice;

trait Buf<T> {
    type Iter<'a>: Iterator<Item = &'a T>
    where
        T: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}

impl<B> Buf<u8> for B
where
    B: AsRef<[u8]>,
{
    type Iter<'a> = slice::Iter<'a, u8>;

    fn iter(&self) -> Self::Iter<'_> {
        self.as_ref().iter()
    }
}

/// A function taking a generic buffer.
fn with_buf<B>(name: &str, buf: B)
where
    B: Buf<u8>,
{
    dbg!(name);

    for b in buf.iter() {
        dbg!(b);
    }
}

fn main() {
    with_buf("vec", vec![1u8, 2, 3, 4]);
    with_buf("slice", &[1u8, 2, 3, 4][..]);
    with_buf("array", [1u8, 2, 3, 4]);
    with_buf("array reference", &[1u8, 2, 3, 4]);
}
