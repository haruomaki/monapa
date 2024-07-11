use monapa::{pdo, Parser, *};

#[derive(Clone, Debug)]
enum Node {
    Paren(Box<Node>, Box<Node>),
    Nil,
}

fn start() -> Parser<Node> {
    pdo!(
        single('(');
        l <- start();
        single(',');
        r <- start();
        single(')');
        return Node::Paren(Box::new(l), Box::new(r))
    ) | pdo!(return Node::Nil)
}

fn main() {
    let parser = start();
    let ast = parser.parse("((,),(,))");
    println!("{:?}", ast);
}
