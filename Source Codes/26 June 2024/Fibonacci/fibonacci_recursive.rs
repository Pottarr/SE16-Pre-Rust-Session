fn fib(count: u32) -> u32 {
    if count == 0 {
        return 1;
    } else {
        if count == 1 {
            return 1;
        } else {
            return fib(count - 1) + fib(count - 2);
        }
    }
}

fn main() {
    let fib_zero = fib(0);
    let fib_one = fib(1);
    let fib_five = fib(5);

    println!("{fib_zero}");
    println!("{fib_one}");
    println!("{fib_five}");
}