//? run --features "{{FEATURE}}" --release

use example::j;

fn main() {
    assert_eq!(j(), 2);
}
