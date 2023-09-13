use monapa::*;
use std::io::{self, Write};

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Expr {
    Hello,
    World,
}

fn main() {
    print!("入力: ");
    io::stdout().flush().unwrap();
    let input = {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");
        buf.trim().to_string()
    };

    let parser_master = pdo! {
        chunk("phone");
        colon <- opt(single(':'));
        single(' ') * (..);
        single('0');
        prefix <- single('7') | single('8') | single('9');
        single('0');
        single('-');
        region <- ascii_digit() * 4;
        single('-');
        id <- ascii_digit() * 4;
        return (colon, prefix, region, id)
    };

    match parser_master.parse(&input) {
        Ok(ast) => println!("受理🎉 {:?}", ast),
        Err(e) => println!("拒否 {:?}", e),
    }
}
