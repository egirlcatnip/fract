Example usage:


```rust
use fract::Fraction;

fn main() {
    let half = Fraction::new(1, 2);
    println!("Half is: {half}");
    let quarter = Fraction::new(1, 4);
    println!("Quarter is: {quarter}");

    let mut value = half + quarter;
    println!("Three quarters is: {value}");

    value /= quarter;
    println!("Three is: {value}");

    let zero = Fraction::ZERO;
    println!("Zero is: {zero}");

    let one = zero + Fraction::ONE;
    println!("One is: {one}");
}

```
