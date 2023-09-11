fn main() {
    let slice = &mut [1, 2, 3 ,4];

    let first = &mut slice[..1];
    let rest = &mut slice[1..];

    println!("{:?}, {:?}", &first, &rest);
}
