#![allow(dead_code)]

use std::ptr;
use std::marker;

pub struct Node {
    value: u32,
    left: Option<ptr::NonNull<Node>>,
    right: Option<ptr::NonNull<Node>>,
    _pin: marker::PhantomPinned,
}

#[no_mangle]
pub fn manipulate_nodes(a: &mut Node, b: &mut Node) {
    a.value += 1;

    unsafe {
        b.left.as_mut().unwrap().as_mut().value += 1;
    }
}
