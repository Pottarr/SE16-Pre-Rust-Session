fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

fn main() {
    let four = factorial(4);
    let one = factorial(1);
    let zero = factorial(0);
    println!("{four}");
    println!("{one}");
    println!("{zero}");
}
