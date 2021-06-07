#![allow(dead_code)]

use std::ptr;
use std::marker;

#[repr(C)]
pub struct Node {
    left: Option<ptr::NonNull<Node>>,
    right: Option<ptr::NonNull<Node>>,
    value: u32,
    _pin: marker::PhantomPinned,
}

impl Node {
    fn value_mut(&mut self) -> &mut u32 {
        unsafe {
            let ptr = self as *mut _ as *mut Option<ptr::NonNull<Node>>;
            let ptr = ptr.add(2) as *mut u32;
            &mut *ptr
        }
    }

    fn left_mut(&mut self) -> &mut Option<ptr::NonNull<Node>> {
        unsafe {
            let ptr = self as *mut _ as *mut Option<ptr::NonNull<Node>>;
            &mut *ptr
        }
    }
}

#[no_mangle]
pub fn manipulate_nodes(a: &mut Node, b: &mut Node) {
    *a.value_mut() += 1;

    unsafe {
        *b.left_mut().unwrap().as_mut().value_mut() += 1;
    }
}
