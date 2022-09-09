pub struct Triple<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

pub fn main() {
    todo!("Make me compile!");
    let mut t = Triple { a: 1, b: 2, c: 3 };

    for e in &t {
        println!("{e}");
    }

    for e in &mut t {
        *e += 10;
    }

    for e in t {
        println!("{e}");
    }
}
