//? run

fn main() {
    let mut i = 0;
    i += 1;
    #[cfg(debug_assertions)]
    { i += 1; }
    assert_eq!(i, 2);
}
