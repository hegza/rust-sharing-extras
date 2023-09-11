fn main() {
    let string = "I am string".to_string();

    function(string);

    // Comment me out to fix the program!
    //println!("{}", string);
}

fn function(string: String) {
    println!("{}", &string);
}
