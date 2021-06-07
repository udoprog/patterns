/// Simple test to see if mutable-noalias works.

#[no_mangle]
pub fn increment(_: &mut i32) {}
