use monapa::*;

fn to_string(vc: Vec<char>) -> String {
    vc.into_iter().collect()
}

// 日本の電話番号をパースする。
fn start() -> Parser<(String, String, String)> {
    pdo! {
        n1 <- numeric() * (1..4); // 市外局番（1〜4桁）
        option(single('-'));
        n2 <- numeric() * (2..4); // 市内局番（2〜4桁）
        option(single('-'));
        n3 <- numeric() * 4; // 加入者番号（4桁）
        return (to_string(n1), to_string(n2), to_string(n3))
    }
}

fn main() {
    let parser = start();

    println!("{:?}", parser.parse("090-1234-5678").unwrap());
}
