fn exponent(base: f64, mut exp: u32) -> f64 {
    if exp == 0 {
        return 1.0;
    } else {
        if exp == 1 {
            return base;
        } else {
            exp -= 1;
             return base * exponent(base ,exp);
        }
    }
}

fn main() {
    let two_three = exponent(2.0, 3);
    let four_four = exponent(4.0, 4);
    let five_zero = exponent(5.0, 0);

    println!("{two_three}");
    println!("{four_four}");
    println!("{five_zero}");
}