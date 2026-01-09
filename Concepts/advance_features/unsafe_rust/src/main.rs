use std::{slice, thread};

static mut COUNTER: u32 = 0;
fn main() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        *r2 = 10;
        println!("r1 is pointing to {}", *r1);
        println!("r2 is pointing to {}", *r2)
    }

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values = unsafe { slice::from_raw_parts_mut(r, 10000) };

    println!("Absolute value of -3 according to C : {}", abs(-3));

    let threads = thread::spawn(|| unsafe {
        add_to_count(3);
        println!("Counter from thread: {}", *(&raw const COUNTER));
    });

    unsafe {
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }

    threads.join().unwrap();
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe extern "C" {
    safe fn abs(input: i32) -> i32;
}

// Threads :: Implementing an unsafe trait
unsafe trait Foo {}

unsafe impl Foo for i32 {}
