// Context-free grammar
// S -> aSb | ε

use monapa::*;

fn start() -> Parser<()> {
    pdo! (
        single('a');
        start();
        single('b');
        return ()
    ) | Parser::ret(())
}

fn main() {
    let parser = start();

    assert!(parser.parse("").is_ok());
    assert!(parser.parse("ab").is_ok());
    assert!(parser.parse("aabb").is_ok());
    assert!(parser.parse("abb").is_err());
    assert!(parser.parse("ac").is_err());
}
