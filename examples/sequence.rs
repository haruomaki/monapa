use monapa::chunk;

fn main() {
    // 並び
    let p1 = chunk("Apple");
    let p2 = chunk("Orange");
    let p3 = p1.bind(move |_| p2.clone());

    assert!(p3.parse("AppleOrange").is_ok());
}
