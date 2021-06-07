#![allow(dead_code)]

use std::marker;
use std::ops;
use std::ptr;

#[repr(transparent)]
struct Aliased<T> {
    target: T,
    _pin: marker::PhantomPinned,
}

impl<T> ops::Deref for Aliased<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.target
    }
}

impl<T> ops::DerefMut for Aliased<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.target
    }
}

pub struct Node {
    value: u32,
    left: Option<Aliased<ptr::NonNull<Node>>>,
    right: Option<Aliased<ptr::NonNull<Node>>>,
    _pin: marker::PhantomPinned,
}

#[no_mangle]
pub fn manipulate_nodes(mut a: ptr::NonNull<Node>, mut b: ptr::NonNull<Node>) {
    unsafe {
        a.as_mut().value += 1;

        if let Some(left) = b.as_mut().left.as_mut() {
            left.as_mut().value += 1;
        }
    }
}
