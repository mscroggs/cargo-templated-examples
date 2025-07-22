//? run --features "{{FEATURE}}" --release

use example1::j;

fn main() {
    assert_eq!(j(), 2);
}
