fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    return result;
}

fn main() {
    let four = factorial(4);
    let one = factorial(1);
    let zero = factorial(0);
    println!("{four}");
    println!("{one}");
    println!("{zero}");
}