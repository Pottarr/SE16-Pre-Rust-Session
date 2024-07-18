fn exponent(base: f64, exp: u32) -> f64 {
    let mut result = 1.0;
    let mut i = 1;
    while i <= exp {
        result *= base;
        i += 1;
    }
    return result;
}

fn main() {
    let two_three = exponent(2.0, 3);
    let four_four = exponent(4.0, 4);
    let five_zero = exponent(5.0, 0);

    println!("{two_three}");
    println!("{four_four}");
    println!("{five_zero}");
}