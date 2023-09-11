use std::{sync::atomic::AtomicBool, thread};

#[derive(Sync)]
struct Mutex {
    is_locked: AtomicBool,
    data: *mut u32,
}

impl Mutex {
    pub fn lock<'l>(&self) -> LockGuard {
        while self.is_locked {}
        let mut data_ref = unsafe {
            &mut (*self.data)
        };
        let mut lock_ref = &mut self.is_locked;
        LockGuard { data: data_ref, lock: lock_ref }
    }
}


struct LockGuard<'d> {
    data: &'d mut u32,
    lock: &'d mut AtomicBool,
}

impl Drop for LockGuard<'_> {
    fn drop(&mut self) {
        *self.lock = AtomicBool::from(false);
    }
}


fn main() {
    let mutex = Mutex {
        is_locked: AtomicBool::from(false),
        data: 42 as * mut _,
    };

    thread::spawn(|| {
        let borrow_mut = mutex.lock();
        *borrow_mut.data = 50;
        println!("{}", &borrow_mut.data);
    });
    thread::spawn(|| {
        let borrow_mut = mutex.lock();
        *borrow_mut.data = 70;
        println!("{}", &borrow_mut.data);
    });
}