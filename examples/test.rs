use std::panic::PanicInfo;

fn test(idx: usize) -> i32 {
    let x = [1, 2, 3, 4];

    // cmp
    // jmp
    x[idx]
}


fn main() {
    let val = test(5);
}
