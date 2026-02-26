#[cfg(feature = "four")]
fn main() {
    assert_eq!(1, 0);
}

#[cfg(not(feature = "four"))]
fn main() {
    assert_eq!(1, 1);
}
