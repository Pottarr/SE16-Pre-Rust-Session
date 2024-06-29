fn main() {
    let mut i = 1;
    let mut j = 1;
    let mut k = 1;
    let height = 6;
    while i <= height {
        while k <= height - i {
            print!(" ");
            k += 1;
        }
        while j <= i {
            print!("*");
            j += 1;
        }
        i += 1;
        k = 1;
        j = 1;
        println!("");
    }
}