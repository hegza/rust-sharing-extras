fn main() {

    let x = 5;

    let y = &mut x;
    let z = &mut x;

    println!("{}", y);
    println!("{}", z);
}
