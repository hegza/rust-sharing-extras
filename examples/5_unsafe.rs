fn main() {
    let ptr = 0x0 as *mut usize;

    unsafe {
        // FREEDOM!
        println!("{}", *ptr);
    }

    println!("Hello?");
}
