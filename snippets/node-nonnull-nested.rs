#![allow(dead_code)]

use std::marker;
use std::ptr;

struct Value {
    inner: u32,
}

pub struct Node {
    value: Value,
    left: Option<ptr::NonNull<Node>>,
    right: Option<ptr::NonNull<Node>>,
    _pin: marker::PhantomPinned,
}

#[no_mangle]
pub fn manipulate_nodes(mut a: ptr::NonNull<Node>) {
    unsafe {
        a.as_mut().value.inner += 1;
    }
}
