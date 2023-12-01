use std::sync::Arc;

fn main() {
    let a = Arc::new([1,2,3]);
    let b = a.clone();

    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
}