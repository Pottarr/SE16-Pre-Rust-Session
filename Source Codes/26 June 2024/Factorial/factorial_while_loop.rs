fn factorial(n: u32) -> u32 {
    let mut i = 1;
    let mut result = 1;
    if n == 0 {
        return 1;
    } else {
        while i <= n {
            result *= i;
            i += 1;
        }
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