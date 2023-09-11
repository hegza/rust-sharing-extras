struct StringBorrower<'s> {
    string: &'s String,
}

fn main() {
    let string = "I am string".to_string();

    let borrower_1 = StringBorrower { string: &string };
    let borrower_2 = StringBorrower { string: &string };

    println!("{}", borrower_1.string);
    println!("{}", borrower_2.string);
}
