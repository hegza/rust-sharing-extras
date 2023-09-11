fn main() {
    let mut floats = vec![1.0f32; 10_000];

    for i in 0..floats.len() {
        // fmul16 // vector
        unsafe { *floats.get_unchecked_mut(i) *= 2f32; }
    }
}