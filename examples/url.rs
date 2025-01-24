use monapa::*;

fn to_string(vc: Vec<char>) -> String {
    vc.into_iter().collect()
}

// The URL path can contain alphabets, numbers, and "."
fn valid_letter() -> Parser<char> {
    alphanumeric() | single('.')
}

fn dir() -> Parser<String> {
    pdo! {
        single('/');
        d <- valid_letter() * (0..);
        return to_string(d)
    }
}

// No pdo version
// fn dir() -> Parser<String> {
//     single('/')
//         .bind(|_| (valid_letter() * (0..)))
//         .map(to_string) // convert from Parser<Vec<char>> into Parser<String>
// }

fn url_path() -> Parser<Vec<String>> {
    pdo! {
        chunk("http");
        single('s') * (0..1);
        chunk(":/");
        path <- dir() * (0..);
        return path
    }
}

fn main() {
    let parser = url_path();

    let path = parser
        .parse("https://codeberg.org/haruomaki/monapa.git")
        .unwrap();
    println!("{:?}", path);
}
