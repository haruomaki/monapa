use monapa::{pdo, Parser, *};

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Node {
    Paren(Vec<Node>),
}

fn start() -> Parser<Node> {
    pdo! {
        single('(');
        parens <- start() * (0..);
        single(')');
        return Node::Paren(parens)
    }
}

fn main() {
    let parser = start();
    let ast = parser.parse("((()())())").unwrap();
    println!("{:?}", ast);
}
