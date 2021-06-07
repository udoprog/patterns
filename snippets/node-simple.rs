#![allow(dead_code)]

use std::ptr;

pub struct Node {
    value: u32,
    left: Option<ptr::NonNull<Node>>,
    right: Option<ptr::NonNull<Node>>,
}

#[no_mangle]
pub fn manipulate_nodes(mut a: ptr::NonNull<Node>, mut b: ptr::NonNull<Node>) {
    unsafe {
        a.as_mut().value += 1;

        if let Some(mut left) = b.as_mut().left {
            left.as_mut().value += 1;
        }
    }
}
