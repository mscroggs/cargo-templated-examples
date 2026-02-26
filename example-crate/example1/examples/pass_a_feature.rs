#[cfg(feature = "three")]
fn main() {
    assert_eq!(1, 1);
}

#[cfg(not(feature = "three"))]
fn main() {
    assert_eq!(1, 0);
}
