fn main() {
    let num = 2;

    if num < 0 {
        if num % 2 == 0 {
            println!("{} is a negative even number", num);
        } else {
            println!("{} is a negative odd number", num);
        }
    } else {
        if num % 2 == 0 {
            println!("{} is a positive even number", num);
        } else {
            println!("{} is a positive odd number", num);
        }
    }
}