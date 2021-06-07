#![allow(dead_code)]

use std::ptr;

pub struct Node {
    value: u32,
    left: Option<ptr::NonNull<Node>>,
    right: Option<ptr::NonNull<Node>>,
}

#[no_mangle]
pub fn manipulate_nodes(a: &mut Node, b: &mut Node) {
    a.value += 1;

    unsafe {
        b.left.as_mut().unwrap().as_mut().value += 1;
    }
}
