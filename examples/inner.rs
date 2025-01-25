use monapa::*;

fn text() -> Parser<String> {
    (alphanumeric() * (0..)).map(|vc| vc.into_iter().collect())
}

fn start() -> Parser<String> {
    single('(')
        .bind(move |_| text().bind(move |t| single(')').bind(move |_| Parser::ret(t.clone()))))
}

// With pdo version
// fn start() -> Parser<String> {
//     pdo! {
//         single('(');
//         t <- text();
//         single(')');
//         return t
//     }
// }

fn main() {
    let parser = start();
    println!("{}", parser.parse("(piyo)").unwrap());
}
