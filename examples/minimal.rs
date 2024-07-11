use monapa::{pdo, single, Parser};

fn main() {
    let parser = single('a').bind(|a| single('b').bind(move |b| Parser::ret(vec![a, b])));
    // let parser = single('a')
    //     .bind(|a| single('b').bind(move |b| single('c').bind(move |c| Parser::ret(vec![a, b, c]))));
    // let parser = pdo! {
    //     a <- single('a');
    //     b <- single('b');
    //     c <- single('c');
    //     return vec![a,b,c]
    // };
    let result = parser.parse("abc");
    println!("{:?}", result);
}
