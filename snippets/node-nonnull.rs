#![allow(dead_code)]

use std::marker;
use std::ptr;

#[repr(transparent)]
struct AliasedNonNull<T> {
    pointer: ptr::NonNull<T>,
    _pin: marker::PhantomPinned,
}

// NB: currently does not work for fat pointers.
impl<T> AliasedNonNull<T> {
    // Implement other used NonNull methods.
    unsafe fn as_mut(&mut self) -> &mut T {
        &mut *(self as *mut _ as *mut T)
    }
}

pub struct Node {
    value: u32,
    left: Option<AliasedNonNull<Node>>,
    right: Option<AliasedNonNull<Node>>,
    _pin: marker::PhantomPinned,
}

#[no_mangle]
pub fn manipulate_nodes(mut a: ptr::NonNull<Node>, mut b: ptr::NonNull<Node>) {
    unsafe {
        a.as_mut().value += 1;
        b.as_mut().left.as_mut().unwrap().as_mut().value += 1;
    }
}
