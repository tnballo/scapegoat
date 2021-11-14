#![no_std]
#![feature(default_alloc_error_handler)]
#![feature(lang_items, start)]
#![feature(rustc_private)]

use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;
use core::ptr::null_mut;

extern crate libc;

use scapegoat::SGMap;

fn main() {
    let mut map: SGMap<usize, usize> = SGMap::new();
    map.insert(1, 2);
    assert_eq!(map.get(&1), Some(&2));
    assert_eq!(map.remove(&1), Some(2));
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
#[no_mangle]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
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
