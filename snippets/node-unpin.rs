#![allow(dead_code)]

use std::marker;
use std::ptr;

pub struct Node {
    value: u32,
    left: Option<ptr::NonNull<Node>>,
    right: Option<ptr::NonNull<Node>>,
    _pin: marker::PhantomPinned,
}

#[no_mangle]
pub fn manipulate_nodes(mut a: ptr::NonNull<Node>, mut b: ptr::NonNull<Node>) {
    unsafe {
        a.as_mut().value += 1;
        b.as_mut().left.as_mut().unwrap().as_mut().value += 1;
    }
}
