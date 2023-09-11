struct StringBorrower<'s> {
    string: &'s mut String,
}

fn main() {
    let mut string = "I am string".to_string();

    let borrower_1 = StringBorrower { string: &mut string };
    let borrower_2 = StringBorrower { string: &mut string };

    println!("{}", borrower_1.string);
    println!("{}", borrower_2.string);
}
