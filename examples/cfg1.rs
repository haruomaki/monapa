// Context free grammer
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

    assert!(parser.parse("aabb").is_ok());
    assert!(parser.parse("aabb").is_ok());
}
