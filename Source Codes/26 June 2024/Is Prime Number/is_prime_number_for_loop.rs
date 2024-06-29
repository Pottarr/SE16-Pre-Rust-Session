fn is_prime(number: u32) -> bool {
    if number == 1{
        return false;
    }
    let mut count = 0;
    for j in 2..number {
        if number % j == 0 {
            count += 1;
        }     
    }
    if count > 0 {
        return false;
    } else {
        return true;
    }

}

fn main() {
    for i in 1..=10 {
        let result: bool = is_prime(i);
        println!("is {i} a prime number?: {result}");
    }
}