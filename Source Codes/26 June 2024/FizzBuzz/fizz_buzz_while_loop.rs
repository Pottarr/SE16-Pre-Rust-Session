fn main() {
    let mut i = 1;
    while i <= 50 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        }else if i % 5  == 0 {
            println!("Buzz");
        } else if i % 3  == 0 {
            println!("Fizz");
        } else {
            println!("{i}")
        }
        i +=1 ;
    }
}