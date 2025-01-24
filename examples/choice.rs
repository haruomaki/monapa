use monapa::chunk;

fn main() {
    let parser = chunk("Apple") | chunk("Orange");

    // Equivalent to:
    // let parser = chunk("Apple").choice(chunk("Orange"));

    println!("{}", parser.parse("Apple").unwrap());
    println!("{}", parser.parse("Orange").unwrap());
    assert!(parser.parse("foo").is_err());
    assert!(parser.parse("Grape").is_err());
}
