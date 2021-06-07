#![allow(dead_code)]

use std::ptr;
use std::marker;

#[repr(transparent)]
struct AliasedNonNull<T> {
    pointer: ptr::NonNull<T>,
    _pin: marker::PhantomPinned,
}

// NB: currently does not work for fat pointers.
impl<T> AliasedNonNull<T> {
    // Overload other used NonNull methods.
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
pub fn manipulate_nodes(a: &mut Node, b: &mut Node) {
    a.value += 1;

    unsafe {
        b.left.as_mut().unwrap().as_mut().value += 1;
    }
}
