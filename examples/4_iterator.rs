fn main() {
    let mut v = vec![1, 2, 3];

    for item in &mut v {
        println!("{}", item);
        v.pop();
    }
}