
trait Example {
    fn foo();
}

struct Concrete1 {}
struct Concrete2 {}

impl Example for Concrete1 {
    fn foo() {
        println!("hello");
    }
}
impl Example for Concrete2 {
    fn foo() {
        println!("moi");
    }
}

fn main() {
    let con = Concrete1 {};
    let con2 = Concrete2 {};

    let con = Box::new( &con as &dyn Example );
    let con2 = Box::new( &con2 as &dyn Example );

    con.foo();
    con2.foo();

    let arr = vec![con, con2];

}