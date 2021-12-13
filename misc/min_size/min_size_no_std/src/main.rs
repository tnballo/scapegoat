#![no_std]
#![no_main]
#![feature(default_alloc_error_handler)]
#![feature(rustc_private)]

use core::panic::PanicInfo;
use core::ptr::null_mut;
use core::alloc::{GlobalAlloc, Layout};

extern crate libc;

use scapegoat::SgMap;

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) -> isize {
    let mut map: SgMap<usize, usize> = SgMap::new();
    map.insert(1, 2);
    assert_eq!(map.get(&1), Some(&2));
    assert_eq!(map.remove(&1), Some(2));
    return 0;
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: PanicAlloc = PanicAlloc;

pub struct PanicAlloc;

unsafe impl GlobalAlloc for PanicAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!()
    }
}